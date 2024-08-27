//@ run-rustfix

struct Foo;

fn From<i32> for Foo {
// { dg-error "" "" { target *-*-* } .-1 }
    fn from(_a: i32) -> Self {
        Foo
    }
}

fn main() {}

