use proc_macro::{TokenStream};
use quote::{quote,quote_spanned};
use syn::{parse_macro_input, DeriveInput, spanned::Spanned};


#[proc_macro_derive(WebType,attributes(require))]
pub fn derive(input:TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    
    // only checks structs
    if let syn::Data::Struct(syn::DataStruct {struct_token:_,fields,semi_token:_}) = input.data {
        // old struct name
        let name = input.ident;
        // new struct name
        let new_name = syn::Ident::new(&format!("Optional{}",name.to_string()), name.span());
        // go over fields and generate Optional field types for given field
        let new_fields = fields.iter().map(|f|{
            let name = f.ident.as_ref().unwrap().clone();
            let ty = &f.ty;
            if helpers::ty_is_option(f){
                quote!{#name: #ty,}
            }else {
                quote!{#name : std::option::Option<#ty>,}
            }
        });
        // generates code to check if the required fields are some at runtime
        let required = fields.iter().map(|f|{
            for attr in &f.attrs {
                if attr.path.segments.iter().next().unwrap().ident == "require" {
                    let name = f.ident.as_ref().unwrap();
                    return quote!{
                        if self.#name.is_none() {
                            return false;
                        };
                    }
                }
            }
            quote!{}
        });


        quote!{
            #[doc = "This is the documentation"]
            #[derive(Default,Debug,serde::Serialize, serde::Deserialize)]
            pub struct #new_name {
                #(#new_fields)*
            }
            impl #new_name {
                #[doc = "This function checks if the required fields are there"]
                pub fn all_required(&self) -> bool {
                    #(#required)*
                    return true
                }
            }
        }.into()
    }else {
        quote_spanned!(input.span() => compile_error!("This Data type is not supported!")).into()
    }
}
