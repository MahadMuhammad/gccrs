//@ force-host
//@ no-prefer-dynamic

#![crate_type = "proc-macro"]
#![allow(warnings)]

extern crate proc_macro;

trait Project {
    type Assoc;
}

#[proc_macro]
pub fn uwu() -> <() as Project>::Assoc {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-error ".E0277." "" { target *-*-* } .-4 }
// { dg-error ".E0277." "" { target *-*-* } .-5 }

