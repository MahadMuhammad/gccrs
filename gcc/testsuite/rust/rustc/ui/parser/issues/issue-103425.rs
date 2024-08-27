fn f() -> f32 {
    3
// { dg-error "" "" { target *-*-* } .-1 }
    5.0
}

fn k() -> f32 {
    2_u32
// { dg-error "" "" { target *-*-* } .-1 }
    3_i8
// { dg-error "" "" { target *-*-* } .-1 }
    5.0
}

fn main() {}

