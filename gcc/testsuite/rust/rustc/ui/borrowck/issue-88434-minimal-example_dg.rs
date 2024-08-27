// Regression test related to issue 88434

const _CONST: &() = &f(&|_| {});
// { dg-error "" "" { target *-*-* } .-1 }

const fn f<F>(_: &F)
where
    F: FnMut(&u8),
{
    panic!() // { dg-error ".E0080." "" { target *-*-* } }
// { dg-error ".E0080." "" { target *-*-* } .-1 }
}

fn main() { }

