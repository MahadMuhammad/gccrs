#![deny(dead_code)]

#[derive(Default)]
struct T; // { dg-error "" "" { target *-*-* } }

#[derive(Default)]
struct Used;

#[derive(Default)]
enum E {
    #[default]
    A,
    B, // { dg-error "" "" { target *-*-* } }
}

// external crate can call T2::default() to construct T2,
// so that no warnings for pub adts
#[derive(Default)]
pub struct T2 {
    _unread: i32,
}

fn main() {
    let _x: Used = Default::default();
}

