#![feature(fn_delegation)]
#![allow(incomplete_features)]

mod to_reuse {
    pub fn consts<const N: i32>() -> i32 {
        N
    }
    pub fn late<'a>(x: &'a u8) -> u8 {
        *x
    }
    pub fn bounds<T: Clone>(_: T) {}
}

// FIXME(fn_delegation): this is supposed to work eventually
reuse to_reuse::consts;
// { dg-error ".E0284." "" { target *-*-* } .-1 }
reuse to_reuse::late;
reuse to_reuse::bounds;

fn main() {
    late::<'static>(&0u8);
// { dg-error ".E0794." "" { target *-*-* } .-1 }

    struct S;
    bounds(S);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

