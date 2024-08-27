// test for #122098 ICE snapshot_vec.rs: index out of bounds: the len is 4 but the index is 4

trait LendingIterator {
    type Item<'q>: 'a;
// { dg-error ".E0261." "" { target *-*-* } .-1 }

    fn for_each(mut self, mut f: Box<dyn FnMut(Self::Item<'_>) + 'static>) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

struct Query<'q> {}
// { dg-error ".E0392." "" { target *-*-* } .-1 }

impl<'static> Query<'q> {
// { dg-error ".E0261." "" { target *-*-* } .-1 }
// { dg-error ".E0261." "" { target *-*-* } .-2 }
    pub fn new() -> Self {}
}

fn data() {
    LendingIterator::for_each(Query::new(&data), Box::new);
// { dg-error ".E0061." "" { target *-*-* } .-1 }
}

pub fn main() {}

