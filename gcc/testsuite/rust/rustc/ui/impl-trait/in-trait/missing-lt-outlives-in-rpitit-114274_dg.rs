trait Iterable {
    type Item<'a>
    where
        Self: 'a;

    fn iter(&self) -> impl Iterator<Item = Self::Item<'missing>>;
// { dg-error ".E0261." "" { target *-*-* } .-1 }
}

fn main() {}

