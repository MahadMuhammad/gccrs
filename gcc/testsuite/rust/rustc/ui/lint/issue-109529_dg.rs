//@ run-rustfix

fn main() {
    for _ in 0..256 as u8 {} // { dg-error "" "" { target *-*-* } }
    for _ in 0..(256 as u8) {} // { dg-error "" "" { target *-*-* } }
}

