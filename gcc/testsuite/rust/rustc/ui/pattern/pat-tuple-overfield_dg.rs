struct S(u8, u8, u8);
struct M(
    u8,
    u8,
    u8,
    u8,
    u8,
);

struct Z0;
struct Z1();
enum E1 {
    Z0,
    Z1(),
}

fn main() {
    match (1, 2, 3) {
        (1, 2, 3, 4) => {} // { dg-error ".E0308." "" { target *-*-* } }
        (1, 2, .., 3, 4) => {} // { dg-error ".E0308." "" { target *-*-* } }
        _ => {}
    }
    match S(1, 2, 3) {
        S(1, 2, 3, 4) => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
        S(1, 2, .., 3, 4) => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
        _ => {}
    }
    match M(1, 2, 3, 4, 5) {
        M(1, 2, 3, 4, 5, 6) => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
        M(1,
          2,
          3,
          4,
          5,
          6) => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
        M(
            1,
            2,
            3,
            4,
            5,
            6,
        ) => {}
// { dg-error ".E0023." "" { target *-*-* } .-2 }
    }
    match Z0 {
        Z0 => {}
        Z0() => {} // { dg-error ".E0532." "" { target *-*-* } }
        Z0(_) => {} // { dg-error ".E0532." "" { target *-*-* } }
        Z0(_, _) => {} // { dg-error ".E0532." "" { target *-*-* } }
    }
    match Z1() {
        Z1 => {} // { dg-error ".E0530." "" { target *-*-* } }
        Z1() => {}
        Z1(_) => {} // { dg-error ".E0023." "" { target *-*-* } }
        Z1(_, _) => {} // { dg-error ".E0023." "" { target *-*-* } }
    }
    match E1::Z0 {
        E1::Z0 => {}
        E1::Z0() => {} // { dg-error ".E0532." "" { target *-*-* } }
        E1::Z0(_) => {} // { dg-error ".E0532." "" { target *-*-* } }
        E1::Z0(_, _) => {} // { dg-error ".E0532." "" { target *-*-* } }
    }
    match E1::Z1() {
        E1::Z1 => {} // { dg-error ".E0532." "" { target *-*-* } }
        E1::Z1() => {}
        E1::Z1(_) => {} // { dg-error ".E0023." "" { target *-*-* } }
        E1::Z1(_, _) => {} // { dg-error ".E0023." "" { target *-*-* } }
    }
}

