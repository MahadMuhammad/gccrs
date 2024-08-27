//@ check-fail

#![feature(linkage)]

extern "C" {
    #[linkage = "extern_weak"]
    static foo: i32;
// { dg-error ".E0791." "" { target *-*-* } .-1 }
}

fn main() {
    println!("{}", unsafe { foo });
}

