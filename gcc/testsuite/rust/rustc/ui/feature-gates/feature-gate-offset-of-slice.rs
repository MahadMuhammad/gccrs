use std::mem::offset_of;

struct S {
    a: u8,
    b: (u8, u8),
    c: [i32],
}

struct T {
    x: i32,
    y: S,
}

type Tup = (i32, [i32]);

fn main() {
    offset_of!(S, c); // { dg-error ".E0277." "" { target *-*-* } }
}

fn other() {
    offset_of!(T, y); // { dg-error ".E0277." "" { target *-*-* } }
}

fn tuple() {
    offset_of!(Tup, 1); // { dg-error ".E0277." "" { target *-*-* } }
}

