#![feature(box_into_inner)]
mod types;
use quote::{quote,quote_spanned};
use syn::{parse_macro_input, DeriveInput, spanned::Spanned};
use std::{io::Write};
use std::fs::File;
use helpers::{extractType};

use crate::types::TSType;

#[proc_macro_derive(TS,attributes(hidden,rename,readonly,ts_type))]
pub fn derive(input:proc_macro::TokenStream) -> proc_macro::TokenStream {
    // this is the struct that is deriving our macro
    let input = parse_macro_input!(input as DeriveInput);
    // create the bindings directory returns compile error if not possible
    match std::fs::create_dir_all("./bindings"){
        Ok(_) => (),
        Err(_) => return quote_spanned!(input.span() => compile_error!("TS macro not applied to this struct")).into(),
    };

    #[allow(unused_assignments)]
    let mut rust_to_ts = String::new();

    // if this if passes we get the fields and tokens of a struct
    if let syn::Data::Struct(syn::DataStruct {struct_token:_,fields,semi_token:_}) = &input.data {
        

        // we iterate over fields and check for types and attributes that may be applied
        let field_types = fields.iter().map(|f|{
            let mut name = match f.ident.as_ref(){
                Some(n) => n.to_string(),
                None => "".into(),
            };
            // loops over attrs if there are any
            for attr in &f.attrs {
                let mut segs = attr.path.segments.iter();
                // this is the name
                let attr_name = segs.next().as_ref().unwrap().ident.to_string();
                if attr_name == "hidden"{
                    // skips itteration thus not creating TS type
                    return None
                }else if attr_name == "readonly" {
                    // add the readonly attribute to field
                    name = "readonly ".to_owned() + &name;
                }
                else if attr_name == "rename" {
                    // the only other one is rename so this renames the TS type
                    let mut token = attr.tokens.clone().into_iter();
                    let symbol = token.next().as_ref().unwrap().to_string();
                    // the new name
                    let new_name = token.next().as_ref().unwrap().to_string();
                    if symbol != "=" {
                        return None
                    }
                    // splice to get rid of the quotations
                    name = new_name[1..new_name.len()-1].to_string()
                };
            }
            
            return Some((name,extractType(f),f.span()))
        });
        
        // some types will have nested structs or enums and need 
        // to be imported to the ts file
        let mut export_list = String::new();
        
        // the string where we create the new TS type
        let mut rust_body = String::new();

        // we loop over fiels and create add the type properties
        for set in field_types{
            if let Some(set) = set {
                if set.0.is_empty() {
                    return quote_spanned!(set.2 => compile_error!("Unamed field structs are not supported, i.e. Tuples")).into();
                }
                match set.1 {
                    Some(ty) => {
                        let size = ty.len();
                        let ts_type:TSType = ty.clone().into();
                        match &ts_type {
                            TSType::Custom(custom_ty) => {
                                let path = format!("./bindings/{}.ts", custom_ty);
                                if !std::path::Path::new(&path).exists() {
                                    return quote_spanned!(set.2 => compile_error!("TS macro not applied to this struct")).into();
                                }else {
                                    export_list += &("import { ".to_owned()+custom_ty+" } from './"+custom_ty+"'\n")
                                }
                            },
                            _ => ()
                        }
                        let ts_string: String = ts_type.into();
                        let is_option = if size > 6 && &ty[0..6] == "Option" { "?: " }else {": "};
                        
                        rust_body += &("\t".to_owned()+ &set.0 + is_option + &ts_string + "\n");
                    },
                    None => return quote_spanned!(set.2 => compile_error!("TS ERROR: Type is not valid")).into()
                }
            }
        };
        // create final formatted string
        let rust_struct_to_ts = format!("{}export type {} = {{\n{}}}", export_list,input.ident.to_string(),rust_body);
        // println!("{rust_struct_to_ts}");
        // asssing it to the variable to be transformed
        rust_to_ts = rust_struct_to_ts;

    }else if let syn::Data::Enum(syn::DataEnum{ enum_token: _,brace_token: _,variants}) = &input.data {
        // the string where we create the new TS type
        let mut rust_body = String::new();

        let mut variants = variants.iter();
        // loop over all variants of enum
        while let Some(variant) = variants.next() {
            // println!("{:?}",variant.ident.to_string());
            rust_body += &("\t".to_owned() + &variant.ident.to_string()+",\n");
        }

        // create final formatted string
        let rust_enum_to_ts = format!("export enum {} {{\n{}}}", input.ident.to_string(),rust_body);
        // println!("{rust_enum_to_ts}");
        rust_to_ts = rust_enum_to_ts;
    }else {
        return quote_spanned!(input.span()=> compile_error!("Data is not a struct")).into()
    }
    // create the file
    let file_name = format!("./bindings/{}.ts",input.ident.to_string());
    let mut w = File::create(&file_name).unwrap();
    // then write to file
    writeln!(&mut w, "{rust_to_ts}").unwrap();
    
    quote!{}.into()
}




