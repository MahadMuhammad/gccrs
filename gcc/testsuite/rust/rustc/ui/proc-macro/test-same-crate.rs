//@ compile-flags: --test
#![crate_type = "proc-macro"]

extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn mac(input: TokenStream) -> TokenStream { loop {} }

#[cfg(test)]
mod test {
    #[test]
    fn t() { crate::mac!(A) }
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

