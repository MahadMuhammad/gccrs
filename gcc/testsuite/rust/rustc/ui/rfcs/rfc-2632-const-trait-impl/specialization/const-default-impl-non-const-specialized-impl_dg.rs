// Tests that specializing trait impls must be at least as const as the default impl.

#![feature(const_trait_impl, effects)] // { dg-warning "" "" { target *-*-* } }
#![feature(min_specialization)]

#[const_trait]
trait Value {
    fn value() -> u32;
}

impl<T> const Value for T {
    default fn value() -> u32 {
        0
    }
}

struct FortyTwo;

impl Value for FortyTwo { // { dg-error "" "" { target *-*-* } }
    fn value() -> u32 {
        println!("You can't do that (constly)");
        42
    }
}

fn main() {}

