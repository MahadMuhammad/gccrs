// Regression test for issue 88434

const _CONST: &[u8] = &f(&[], |_| {});
// { dg-error "" "" { target *-*-* } .-1 }

const fn f<F>(_: &[u8], _: F) -> &[u8]
where
    F: FnMut(&u8),
{
    panic!() // { dg-error ".E0080." "" { target *-*-* } }
// { dg-error ".E0080." "" { target *-*-* } .-1 }
}

fn main() { }

