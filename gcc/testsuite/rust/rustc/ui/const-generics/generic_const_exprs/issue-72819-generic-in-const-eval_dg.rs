// Regression test for #72819: ICE due to failure in resolving the const generic in `Arr`'s type
// bounds.
//@ revisions: full min
#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

struct Arr<const N: usize>
where Assert::<{N < usize::MAX / 2}>: IsTrue,
// { dg-error "" "" { target *-*-* } .-1 }
{
}

enum Assert<const CHECK: bool> {}

trait IsTrue {}

impl IsTrue for Assert<true> {}

fn main() {
    let x: Arr<{usize::MAX}> = Arr {};
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

