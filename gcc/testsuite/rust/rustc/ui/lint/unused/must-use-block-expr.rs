//@ run-rustfix
//@ check-pass

#![warn(unused_must_use)]

#[must_use]
fn foo() -> i32 {
    42
}

fn bar() {
    {
        foo();
// { dg-warning "" "" { target *-*-* } .-1 }
    }
}

fn baz() {
    {
        foo()
// { dg-warning "" "" { target *-*-* } .-1 }
    };
}

fn main() {
    bar();
    baz();
    {
        1 + 2;
// { dg-warning "" "" { target *-*-* } .-1 }
    }
    {
        1 + 2
// { dg-warning "" "" { target *-*-* } .-1 }
    };
}

