//@ check-fail

#![feature(linkage)]

extern "C" {
    #[linkage = "foo"]
    static foo: *const i32;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    println!("{:?}", unsafe { foo });
}

