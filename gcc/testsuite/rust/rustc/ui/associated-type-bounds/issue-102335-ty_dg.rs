trait T {
    type A: S<C<i32 = u32> = ()>; // Just one erroneous equality constraint
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { dg-error ".E0229." "" { target *-*-* } .-2 }
}

trait T2 {
    type A: S<C<i32 = u32, X = i32> = ()>; // More than one erroneous equality constraints
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { dg-error ".E0229." "" { target *-*-* } .-2 }
}

trait Q {}

trait S {
    type C: Q;
}

fn main() {}

