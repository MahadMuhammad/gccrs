// #29924

trait Trait {
    const N: usize;
}

impl dyn Trait {
// { dg-error ".E0038." "" { target *-*-* } .-1 }
    const fn n() -> usize { Self::N }
// { dg-error ".E0038." "" { target *-*-* } .-1 }
}

fn main() {}

