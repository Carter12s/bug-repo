use proc_macro::TokenStream;


#[proc_macro]
pub fn what_is_working_dir(_input: TokenStream) -> TokenStream {
    let wd = crate2::get_working_dir();
    quote::quote!( #wd ).into()
}