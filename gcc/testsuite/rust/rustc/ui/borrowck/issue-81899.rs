// Regression test for #81899.
// The `panic!()` below is important to trigger the fixed ICE.

const _CONST: &[u8] = &f(&[], |_| {});
// { dg-error "" "" { target *-*-* } .-1 }

const fn f<F>(_: &[u8], _: F) -> &[u8]
where
    F: FnMut(&u8),
{
    panic!() // { dg-error ".E0080." "" { target *-*-* } }
// { dg-error ".E0080." "" { target *-*-* } .-1 }
}

fn main() {}

