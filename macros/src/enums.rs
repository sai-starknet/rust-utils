use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields};

pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let enum_name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let variants = match &input.data {
        syn::Data::Enum(data) => &data.variants,
        _ => {
            return syn::Error::new_spanned(&input, "EnumFrom can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let impls = variants.iter().filter_map(|variant| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                let inner_ty = &fields.unnamed.first().unwrap().ty;
                Some(quote! {
                    impl #impl_generics From<#inner_ty> for #enum_name #ty_generics #where_clause {
                        fn from(value: #inner_ty) -> Self {
                            Self::#variant_name(value)
                        }
                    }
                })
            }
            _ => None,
        }
    });

    let expanded = quote! {
        #(#impls)*
    };

    TokenStream::from(expanded)
}
