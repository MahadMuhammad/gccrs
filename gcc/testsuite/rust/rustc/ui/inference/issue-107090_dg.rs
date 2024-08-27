use std::marker::PhantomData;
struct Foo<'a, 'b, T>(PhantomData<(&'a (), &'b (), T)>)
where
    Foo<'short, 'out, T>: Convert<'a, 'b>;
// { dg-error ".E0261." "" { target *-*-* } .-1 }
// { dg-error ".E0261." "" { target *-*-* } .-2 }

trait Convert<'a, 'b>: Sized {
    fn cast(&'a self) -> &'b Self;
}
impl<'long: 'short, 'short, T> Convert<'long, 'b> for Foo<'short, 'out, T> {
// { dg-error ".E0261." "" { target *-*-* } .-1 }
// { dg-error ".E0261." "" { target *-*-* } .-2 }
    fn cast(&'long self) -> &'short Foo<'short, 'out, T> {
// { dg-error ".E0261." "" { target *-*-* } .-1 }
        self
    }
}

fn badboi<'in_, 'out, T>(x: Foo<'in_, 'out, T>, sadness: &'in_ Foo<'short, 'out, T>) -> &'out T {
// { dg-error ".E0261." "" { target *-*-* } .-1 }
    sadness.cast()
}

fn main() {}

