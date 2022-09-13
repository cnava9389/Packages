use syn::{PathArguments, GenericArgument};

pub fn extractType(field:&syn::Field) -> Option<String> {
    let syn::Field { attrs, vis:_, ident: _, colon_token:_, ty } = field;
    
    if let syn::Type::Path(ty_path) = ty {
        // the path that contains the first or only part of the type
        let seg = ty_path.path.segments.iter().next().unwrap();
        // this turns the type into a string then we compare
        let seg_type = if !attrs.is_empty() {
            let mut new_type = String::new();
            for attr in attrs {
                let mut segs = attr.path.segments.iter();
                // this is the name
                let attr_name = segs.next().as_ref().unwrap().ident.to_string();
                if attr_name == "ts_type" {
                    // the only other one is rename so this renames the TS type
                    let mut token = attr.tokens.clone().into_iter();
                    // the new name
                    let new_name = token.next().as_ref().unwrap().to_string();
                    if let Some( symbol) = token.next().as_ref() {
                        if symbol.to_string() != "=" {
                            return None
                        }
                        // splice to get rid of the quotations
                        new_type = new_name[1..new_name.len()-1].to_string();
                    } 
                }else {
                    new_type = seg.ident.to_string();
                }
            }
            new_type
        }else {
            seg.ident.to_string()
        };

        if seg_type == "Result"{
            return None;
        } else if seg_type != "Option" && seg_type != "Vec" {
            return  Some(seg_type);
        }
        
        let type_params = &seg.arguments;
        // It should have only on angle-bracketed param ("<String>"): except Result
        let generic_arg = match type_params {
            PathArguments::AngleBracketed(params) => params.args.iter().next().unwrap(),
            ty => return None
        };
        // This argument must be a type:
        match generic_arg {
            GenericArgument::Type(ty) => {
                if let syn::Type::Path(syn::TypePath { qself:_, path }) = ty {
                    let first = path.segments.iter().next().unwrap();
                    if first.ident.to_string() == "Vec"{
                        match &first.arguments {
                            PathArguments::AngleBracketed(params) =>{
                                let arg = params.args.iter().next().unwrap();
                                match arg {
                                    GenericArgument::Type(ty)=> {
                                        if let syn::Type::Path(syn::TypePath {qself:_,path}) = ty {
                                            // println!("{}",path.get_ident().unwrap().to_string())
                                            return Some(seg_type+"<Vec<"+&path.get_ident().unwrap().to_string()+">>");
                                        };
                                    },
                                    _ => ()
                                };
                            },
                            _ => () 
                        }

                    }
                    return Some(seg_type+"<"+&first.ident.to_string()+">")
                }

            },
            s => return None
        }
    }
    None
}

pub fn ty_is_option(f:&syn::Field) -> bool{
    if let syn::Type::Path(syn::TypePath { qself:_, path }) = &f.ty {
        return path.segments.len() == 1 && path.segments[0].ident == "Option";
    }
    false
}