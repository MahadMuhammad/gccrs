//@ run-rustfix
#[derive(Debug, Default, Eq, PartialEq)]
struct A {
    b: u32,
    c: u64,
    d: usize,
}

fn a() {
    let q = A { c: 5..Default::default() };
// { dg-error ".E0063." "" { target *-*-* } .-1 }
// { help ".E0063." "" { target *-*-* } .-2 }
    let r = A { c: 5, ..Default::default() };
    assert_eq!(q, r);
}

#[derive(Debug, Default, Eq, PartialEq)]
struct B {
    b: u32,
}

fn b() {
    let q = B { b: 1..Default::default() };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let r = B { b: 1 };
    assert_eq!(q, r);
}

fn main() {
    a();
    b();
}

