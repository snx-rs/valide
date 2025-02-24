use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Validate, attributes(rule))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    todo!()
}
