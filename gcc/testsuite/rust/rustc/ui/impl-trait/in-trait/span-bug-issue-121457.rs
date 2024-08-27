pub trait Iterable {
    type Item<'a>
    where
        Self: 'a;

    fn iter(&self) -> impl Iterator;
}

impl<'a, I: 'a + Iterable> Iterable for &'a I {
    type Item = u32;
// { dg-error ".E0195." "" { target *-*-* } .-1 }

    fn iter(&self) -> impl for<'missing> Iterator<Item = Self::Item<'missing>> {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
}

fn main() {}

