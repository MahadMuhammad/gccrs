//@ run-rustfix

struct S<T>(T);
struct S2;

impl<T: Default> impl Default for S<T> {
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    fn default() -> Self { todo!() }
}

impl impl Default for S2 {
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    fn default() -> Self { todo!() }
}


fn main() {}

