//@ build-fail
//@ compile-flags: -Zmir-opt-level=3

fn foo<T>() {
    if false {
        const { panic!() } // { dg-error ".E0080." "" { target *-*-* } }
    }
}

fn main() {
    foo::<i32>();
}

