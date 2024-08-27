//@ run-rustfix

struct A {

}

trait M {
    fn foo(_a: Self);
    fn bar(_a: Self);
    fn baz(_a: i32);
}

impl M for A {
    fn foo(_a: Self) {}
    fn bar(_a: A) {}
    fn baz(_a: i32) {}
}

fn main() {
    let _a = A {};
    _a.foo();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    _a.baz(0);
// { dg-error ".E0599." "" { target *-*-* } .-1 }

    let _b = A {};
    _b.bar();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

