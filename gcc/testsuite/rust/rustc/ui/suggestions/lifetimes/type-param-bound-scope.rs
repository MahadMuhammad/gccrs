// Make sure we suggest the bound `T: 'a` in the correct scope:
// trait, impl or associated fn.
//@ run-rustfix
#![allow(dead_code)]

struct Inv<'a>(#[allow(dead_code)] Option<*mut &'a u8>);

fn check_bound<'a, A: 'a>(_: A, _: Inv<'a>) {}

trait Trait1<'a>: Sized {
    fn foo(self, lt: Inv<'a>) {
        check_bound(self, lt)
// { dg-error ".E0309." "" { target *-*-* } .-1 }
    }
}

trait Trait2: Sized {
    fn foo<'a>(self, lt: Inv<'a>) {
        check_bound(self, lt)
// { dg-error ".E0309." "" { target *-*-* } .-1 }
    }
}

trait Trait3<T> {
    fn foo<'a>(arg: T, lt: Inv<'a>) {
        check_bound(arg, lt)
// { dg-error ".E0309." "" { target *-*-* } .-1 }
    }
}

trait Trait4<'a> {
    fn foo<T>(arg: T, lt: Inv<'a>) {
        check_bound(arg, lt)
// { dg-error ".E0309." "" { target *-*-* } .-1 }
    }
}

trait Trait5<'a> {
    fn foo(self, _: Inv<'a>);
}
impl<'a, T> Trait5<'a> for T {
    fn foo(self, lt: Inv<'a>) {
        check_bound(self, lt);
// { dg-error ".E0309." "" { target *-*-* } .-1 }
    }
}

fn main() {}

