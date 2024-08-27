//@ run-rustfix

struct Foo {
    bar: Bar,
}

struct Bar {
    qux: i32,
}

pub fn post_regular() {
    let mut i = 0;
    i++; // { dg-error "" "" { target *-*-* } }
    println!("{}", i);
}

pub fn post_while() {
    let mut i = 0;
    while i++ < 5 {
// { dg-error "" "" { target *-*-* } .-1 }
        println!("{}", i);
    }
}

pub fn post_regular_tmp() {
    let mut tmp = 0;
    tmp++; // { dg-error "" "" { target *-*-* } }
    println!("{}", tmp);
}

pub fn post_while_tmp() {
    let mut tmp = 0;
    while tmp++ < 5 {
// { dg-error "" "" { target *-*-* } .-1 }
        println!("{}", tmp);
    }
}

pub fn post_field() {
    let mut foo = Foo { bar: Bar { qux: 0 } };
    foo.bar.qux++;
// { dg-error "" "" { target *-*-* } .-1 }
    println!("{}", foo.bar.qux);
}

pub fn post_field_tmp() {
    struct S {
        tmp: i32
    }
    let mut s = S { tmp: 0 };
    s.tmp++;
// { dg-error "" "" { target *-*-* } .-1 }
    println!("{}", s.tmp);
}

pub fn pre_field() {
    let mut foo = Foo { bar: Bar { qux: 0 } };
    ++foo.bar.qux;
// { dg-error "" "" { target *-*-* } .-1 }
    println!("{}", foo.bar.qux);
}

fn main() {}

