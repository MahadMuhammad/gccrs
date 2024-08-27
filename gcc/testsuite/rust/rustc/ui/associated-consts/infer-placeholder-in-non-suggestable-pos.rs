trait Trait {
    const ASSOC: i32;
}

impl Trait for () {
    const ASSOC: &dyn Fn(_) = 1i32;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}

fn main() {}

