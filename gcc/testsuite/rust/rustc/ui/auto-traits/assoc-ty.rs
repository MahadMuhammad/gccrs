//@ revisions: current next
//@[next] compile-flags: -Znext-solver
//@ ignore-compare-mode-next-solver (explicit revisions)

// Tests that projection doesn't explode if we accidentally
// put an associated type on an auto trait.

auto trait Trait {
// { dg-error "" "" { target *-*-* } .-1 }
    type Output;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {
    let _: <() as Trait>::Output = ();
// { dg-error "" "" { target *-*-* } .-1 }
}

