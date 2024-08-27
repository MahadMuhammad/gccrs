//@ run-rustfix

struct A {}

impl A {
    fn hello(_a: i32) {}
    fn test(_a: Self, _b: i32) {}
}

struct B<T> {
    _b: T
}
impl<T> B<T> {
    fn hello(_a: i32) {}
    fn test(_a: Self, _b: i32) {}
}

fn main() {
    let _a = A {};
    _a.hello(1);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    _a.test(1);
// { dg-error ".E0599." "" { target *-*-* } .-1 }

    let _b = B {_b: ""};
    _b.hello(1);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    _b.test(1);
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

