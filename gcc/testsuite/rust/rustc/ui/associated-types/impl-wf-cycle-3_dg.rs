trait A<T> {}

trait B {
    type Type;
}

impl<T> B for T // { dg-error ".E0275." "" { target *-*-* } }
where
    T: A<Self::Type>,
{
    type Type = bool;
}
fn main() {}

