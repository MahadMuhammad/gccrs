#[derive(Clone)]
struct S;

// without Clone
struct T;

fn foo(_: S) {}

fn test1() {
    let s = &S;
    foo(s); // { dg-error ".E0308." "" { target *-*-* } }
}

fn bar(_: T) {}
fn test2() {
    let t = &T;
    bar(t); // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {
    test1();
    test2();
}

