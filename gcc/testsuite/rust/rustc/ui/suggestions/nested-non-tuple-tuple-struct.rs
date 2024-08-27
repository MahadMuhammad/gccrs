pub struct S(f32, f32);

pub enum E {
    V(f32, f32),
}

fn main() {
    let _x = (S { x: 1.0, y: 2.0 }, S { x: 3.0, y: 4.0 });
// { dg-error ".E0560." "" { target *-*-* } .-1 }
// { dg-error ".E0560." "" { target *-*-* } .-2 }
// { dg-error ".E0560." "" { target *-*-* } .-3 }
// { dg-error ".E0560." "" { target *-*-* } .-4 }
    let _y = (E::V { x: 1.0, y: 2.0 }, E::V { x: 3.0, y: 4.0 });
// { dg-error ".E0559." "" { target *-*-* } .-1 }
// { dg-error ".E0559." "" { target *-*-* } .-2 }
// { dg-error ".E0559." "" { target *-*-* } .-3 }
// { dg-error ".E0559." "" { target *-*-* } .-4 }
}

