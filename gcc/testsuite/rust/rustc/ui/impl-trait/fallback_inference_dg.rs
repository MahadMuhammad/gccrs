use std::marker::PhantomData;

fn weird() -> PhantomData<impl Sized> {
    PhantomData // { dg-error ".E0282." "" { target *-*-* } }
}

fn main() {}

