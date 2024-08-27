// Regression test for #79714

trait Grault {
    type A;
}

impl<T: Grault> Grault for (T,)
// { dg-error ".E0275." "" { target *-*-* } .-1 }
where
    Self::A: Copy,
{
    type A = ();
}

fn main() {}

