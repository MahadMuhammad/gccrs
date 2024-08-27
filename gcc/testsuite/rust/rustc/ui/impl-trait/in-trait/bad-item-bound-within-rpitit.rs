// issue: 114145


pub trait Iterable {
    type Item<'a>
    where
        Self: 'a;

    fn iter(&self) -> impl '_ + Iterator<Item = Self::Item<'_>>;
}

impl<'a, I: 'a + Iterable> Iterable for &'a I {
    type Item<'b> = I::Item<'a>
    where
        'b: 'a;
// { dg-error ".E0276." "" { target *-*-* } .-1 }

    fn iter(&self) -> impl 'a + Iterator<Item = I::Item<'a>> {
// { dg-warning "" "" { target *-*-* } .-1 }
        (*self).iter()
    }
}

fn main() {}

