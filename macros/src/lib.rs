mod enums;

use proc_macro::TokenStream;

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    enums::derive_enum_from(input)
}
