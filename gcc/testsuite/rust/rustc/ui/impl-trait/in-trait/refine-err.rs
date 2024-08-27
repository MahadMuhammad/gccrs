#![deny(refining_impl_trait)]

trait FromRow {
    fn prepare(self) -> impl Fn() -> T;
// { dg-error ".E0412." "" { target *-*-* } .-1 }
}

impl<T> FromRow for T {
    fn prepare(self) -> impl Fn() -> T {
        || todo!()
    }
}

fn main() {}

