extern "C" {
    type Item = [T] where [T]: Sized;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }
// { dg-error ".E0658." "" { target *-*-* } .-4 }
// { dg-error ".E0658." "" { target *-*-* } .-5 }
}

fn main() {}

