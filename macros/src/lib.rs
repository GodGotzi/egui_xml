extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn form(input: TokenStream) -> TokenStream {
    input
}
