pub trait ToNbt<T> {
    fn new(val: T) -> Self;
}

impl dyn ToNbt<Self> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

