// { dg-error "" "" { target *-*-* } }

pub trait Gen<'source> {
    type Output;

    fn gen<T>(&self) -> T
    where
        Self: for<'s> Gen<'s, Output = T>;
}

fn main() {}

