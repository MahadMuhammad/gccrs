#![allow(incomplete_features)]
#![feature(unnamed_fields)]

struct A { // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
    _: struct { // { dg-note "" "" { target *-*-* } }
        a: i32,
    },
    _: struct { // { dg-note "" "" { target *-*-* } }
        _: struct {
            b: i32,
        },
    },
}

union B { // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
    _: union { // { dg-note "" "" { target *-*-* } }
        a: i32,
    },
    _: union { // { dg-note "" "" { target *-*-* } }
        _: union {
            b: i32,
        },
    },
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Foo {}

#[derive(Clone, Copy)]
struct Bar {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }

struct C { // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
    _: Foo, // { dg-note "" "" { target *-*-* } }
}

union D { // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
    _: Foo, // { dg-note "" "" { target *-*-* } }
}

#[repr(C)]
struct E {
    _: Bar, // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
    _: struct {
        _: Bar, // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
    },
}

#[repr(C)]
union F {
    _: Bar, // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
    _: union {
        _: Bar, // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
    },
}

fn main() {}

