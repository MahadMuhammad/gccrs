#![deny(dead_code)]

use std::mem::offset_of;

struct Alpha {
    a: (),
    b: (), // { dg-error "" "" { target *-*-* } }
    c: Beta,
}

struct Beta {
    a: (), // { dg-error "" "" { target *-*-* } }
    b: (),
}

struct Gamma {
    a: (), // { dg-error "" "" { target *-*-* } }
    b: (),
}

struct Delta {
    a: (),
    b: (), // { dg-error "" "" { target *-*-* } }
}

trait Trait {
    type Assoc;
}
impl Trait for () {
    type Assoc = Delta;
}

struct Project<T: Trait> {
    a: u8, // { dg-error "" "" { target *-*-* } }
    b: <T as Trait>::Assoc,
}

fn main() {
    offset_of!(Alpha, a);
    offset_of!(Alpha, c.b);
    offset_of!((Gamma,), 0.b);
    offset_of!(Project::<()>, b.a);
}

