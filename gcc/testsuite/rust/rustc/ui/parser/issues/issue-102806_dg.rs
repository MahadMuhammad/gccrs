#![allow(dead_code)]

#[derive(Default)]
struct V3 {
    x: f32,
    y: f32,
    z: f32,
}

fn pz(v: V3) {
    let _ = V3 { z: 0.0, ...v};
// { dg-error "" "" { target *-*-* } .-1 }

    let _ = V3 { z: 0.0, ...Default::default() };
// { dg-error "" "" { target *-*-* } .-1 }

    let _ = V3 { z: 0.0, ... };
// { dg-error ".E0063." "" { target *-*-* } .-1 }
// { dg-error ".E0063." "" { target *-*-* } .-2 }

    let V3 { z: val, ... } = v;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

