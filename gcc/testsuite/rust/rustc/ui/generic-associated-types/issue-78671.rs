//@ revisions: base extended

#![cfg_attr(extended, feature(generic_associated_types_extended))]
#![cfg_attr(extended, allow(incomplete_features))]

trait CollectionFamily {
    type Member<T>;
}
fn floatify() {
    Box::new(Family) as &dyn CollectionFamily<Member=usize>
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

struct Family;

fn main() {}

