//@ compile-flags: --crate-type lib -O -C debug-assertions=yes

// Regression test for issue 118786

macro_rules! make_macro {
    ($macro_name:tt) => {
        macro_rules! $macro_name {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
            () => {}
        }
    }
}

make_macro!((meow));
// { dg-error "" "" { target *-*-* } .-1 }

