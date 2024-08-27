#![feature(if_let_guard)]
#![allow(unused, non_snake_case)]

enum E {
    A,
}

#[allow(bindings_with_variant_name, irrefutable_let_patterns)]
fn foo() {
    match E::A {
        #[deny(bindings_with_variant_name)]
        A => {}
// { dg-error ".E0170." "" { target *-*-* } .-1 }
    }

    match &E::A {
        #[deny(irrefutable_let_patterns)]
        a if let b = a => {}
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }
}

fn main() { }

