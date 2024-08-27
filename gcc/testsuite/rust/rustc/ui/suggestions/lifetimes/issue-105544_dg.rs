//@ run-rustfix

#![allow(warnings)]

fn foo(d: impl Sized, p: &mut ()) -> impl Sized + '_ { // { dg-note "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    (d, p)
// { dg-error ".E0309." "" { target *-*-* } .-1 }
// { dg-note ".E0311." "" { target *-*-* } .-2 }
}

fn foo1<'b>(d: impl Sized, p: &'b mut ()) -> impl Sized + '_ {
// { dg-note "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    (d, p) // { dg-note ".E0309." "" { target *-*-* } }
// { dg-error ".E0309." "" { target *-*-* } .-1 }
}

fn foo2<'a>(d: impl Sized + 'a, p: &mut ()) -> impl Sized + '_ { // { dg-note "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    (d, p)
// { dg-error ".E0311." "" { target *-*-* } .-1 }
// { dg-note ".E0311." "" { target *-*-* } .-2 }
}

fn bar<T : Sized>(d: T, p: & mut ()) -> impl Sized + '_ { // { dg-note "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    (d, p)
// { dg-error ".E0311." "" { target *-*-* } .-1 }
// { dg-note ".E0311." "" { target *-*-* } .-2 }
}

fn bar1<'b, T : Sized>(d: T, p: &'b mut ()) -> impl Sized + '_ {
// { dg-note "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    (d, p) // { dg-note ".E0309." "" { target *-*-* } }
// { dg-error ".E0311." "" { target *-*-* } .-1 }
}

fn bar2<'a, T : Sized + 'a>(d: T, p: &mut ()) -> impl Sized + '_ { // { dg-note "" "" { target *-*-* } }
// { help "" "" { target *-*-* } .-1 }
    (d, p)
// { dg-error ".E0311." "" { target *-*-* } .-1 }
// { dg-note ".E0311." "" { target *-*-* } .-2 }
}

fn main() {}

