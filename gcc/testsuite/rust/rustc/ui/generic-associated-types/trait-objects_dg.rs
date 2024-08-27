//@ revisions: base extended

#![cfg_attr(extended, feature(generic_associated_types_extended))]
#![cfg_attr(extended, allow(incomplete_features))]

trait StreamingIterator {
    type Item<'a> where Self: 'a;
    fn size_hint(&self) -> (usize, Option<usize>);
    // Uncommenting makes `StreamingIterator` not object safe
//    fn next(&mut self) -> Self::Item<'_>;
}

fn min_size(x: &mut dyn for<'a> StreamingIterator<Item<'a> = &'a i32>) -> usize {
// { dg-error "" "" { target *-*-* } .-1 }
    x.size_hint().0
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {}

