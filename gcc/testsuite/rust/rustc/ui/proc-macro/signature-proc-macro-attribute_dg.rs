//@ force-host
//@ no-prefer-dynamic

#![crate_type = "proc-macro"]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bad_input(input: String) -> TokenStream {
// { dg-error "" "" { target *-*-* } .-1 }
    ::proc_macro::TokenStream::new()
}

#[proc_macro_attribute]
pub fn bad_output(input: TokenStream) -> String {
// { dg-error "" "" { target *-*-* } .-1 }
    String::from("blah")
}

#[proc_macro_attribute]
pub fn bad_everything(input: String) -> String {
// { dg-error "" "" { target *-*-* } .-1 }
    input
}

#[proc_macro_attribute]
pub fn too_many(a: TokenStream, b: TokenStream, c: String) -> TokenStream {
// { dg-error "" "" { target *-*-* } .-1 }
}

