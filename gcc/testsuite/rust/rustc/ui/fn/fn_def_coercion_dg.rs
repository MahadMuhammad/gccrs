//! Test that coercing between function items of the same function,
//! but with different generic args succeeds in typeck, but then fails
//! in borrowck when the lifetimes can't actually be merged.

fn foo<T>(t: T) -> T {
    t
}

fn f<'a, 'b, 'c: 'a + 'b>(a: &'a (), b: &'b (), c: &'c ()) {
    let mut x = foo::<&'a ()>; // { dg-error "" "" { target *-*-* } }
    x = foo::<&'b ()>; // { dg-error "" "" { target *-*-* } }
    x = foo::<&'c ()>;
    x(a);
    x(b);
    x(c);
}

fn g<'a, 'b, 'c: 'a + 'b>(a: &'a (), b: &'b (), c: &'c ()) {
    let x = foo::<&'c ()>;
    let _: &'c () = x(a); // { dg-error "" "" { target *-*-* } }
}

fn h<'a, 'b, 'c: 'a + 'b>(a: &'a (), b: &'b (), c: &'c ()) {
    let x = foo::<&'a ()>;
    let _: &'a () = x(c);
}

fn i<'a, 'b, 'c: 'a + 'b>(a: &'a (), b: &'b (), c: &'c ()) {
    let mut x = foo::<&'c ()>;
    x = foo::<&'b ()>; // { dg-error "" "" { target *-*-* } }
    x = foo::<&'a ()>; // { dg-error "" "" { target *-*-* } }
    x(a);
    x(b);
    x(c);
}

fn j<'a, 'b, 'c: 'a + 'b>(a: &'a (), b: &'b (), c: &'c ()) {
    let x = match true {
        true => foo::<&'b ()>,  // { dg-error "" "" { target *-*-* } }
        false => foo::<&'a ()>, // { dg-error "" "" { target *-*-* } }
    };
    x(a);
    x(b);
    x(c);
}

fn k<'a, 'b, 'c: 'a + 'b>(a: &'a (), b: &'b (), c: &'c ()) {
    let x = match true {
        true => foo::<&'c ()>,
        false => foo::<&'a ()>, // { dg-error "" "" { target *-*-* } }
    };

    x(a);
    x(b); // { dg-error "" "" { target *-*-* } }
    x(c);
}

fn main() {}

