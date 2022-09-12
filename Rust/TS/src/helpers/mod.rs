use syn::{PathArguments, GenericArgument};

pub fn extractType(field:&syn::Field) -> Option<String> {
    let syn::Field { attrs:_, vis:_, ident, colon_token:_, ty } = field;
    let ident = match ident{
        Some(n) => n.to_string(),
        None => "NO NAME".into(),
    };
    
    if let syn::Type::Path(ty_path) = ty {
        // the path that contains the first or only part of the type
        let seg = ty_path.path.segments.iter().next().unwrap();
        // this turns the type into a string then we compare
        let seg_type = seg.ident.to_string();
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