
extern crate proc_macro;

#[proc_macro]
pub fn my_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dbg!(input)
}
