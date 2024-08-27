#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

pub fn bar()
where
    for<const N: usize = {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    (||1usize)()
}> V: IntoIterator
// { dg-error ".E0412." "" { target *-*-* } .-1 }
{
}

fn main() {
    bar();
}

