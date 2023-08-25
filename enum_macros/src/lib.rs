extern crate proc_macro;
use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use syn::{ parse_macro_input, DeriveInput, Data, Fields, Ident };


#[proc_macro_derive(EnumArray)]
pub fn enum_array(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    // Check if the input is an enum
    let data = match input.data {
        Data::Enum(data) => data,
        _ => panic!("Only structs are supported")
    };
    
    let mut variant_names: Vec<_> = Vec::new();
    for variant in data.variants.iter() {
        match variant.fields {
            Fields::Unit => {},
            _ => panic!("EnumArray can only be derived for enums with unit variants")
        }
        let variant_name = &variant.ident;
        variant_names.push(variant_name);
    }

    let length = variant_names.len();

    let enum_array: proc_macro2::TokenStream = quote! {
        impl #name {
            pub const VARIANTS: [#name; #length] = [
                #(#name::#variant_names,)*
            ];
        }
    };

    println!("{}", enum_array);
    return TokenStream::from(enum_array);
}

