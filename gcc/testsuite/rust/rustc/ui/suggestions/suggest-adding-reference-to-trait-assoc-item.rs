//@ run-rustfix
#![allow(unused_variables)]

fn foo(foo: &mut usize) {
    todo!()
}

fn bar(bar: &usize) {
    todo!()
}

fn main() {
    foo(Default::default()); // { dg-error ".E0277." "" { target *-*-* } }
    bar(Default::default()); // { dg-error ".E0277." "" { target *-*-* } }
}

