enum Enum {
// { dg-error ".E0081." "" { target *-*-* } .-1 }
    P = 3,
// { dg-note "" "" { target *-*-* } .-1 }
    X = 3,
// { dg-note "" "" { target *-*-* } .-1 }
    Y = 5
}

#[repr(u8)]
enum EnumOverflowRepr {
// { dg-error ".E0081." "" { target *-*-* } .-1 }
    P = 257,
// { dg-note "" "" { target *-*-* } .-1 }
    X = 513,
// { dg-note "" "" { target *-*-* } .-1 }
}

#[repr(i8)]
enum NegDisEnum {
// { dg-error ".E0081." "" { target *-*-* } .-1 }
    First = -1,
// { dg-note "" "" { target *-*-* } .-1 }
    Second = -2,
// { dg-note "" "" { target *-*-* } .-1 }
    Last,
// { dg-note "" "" { target *-*-* } .-1 }
}

enum MultipleDuplicates {
// { dg-error ".E0081." "" { target *-*-* } .-1 }
// { dg-error ".E0081." "" { target *-*-* } .-2 }
    V0,
// { dg-note "" "" { target *-*-* } .-1 }
    V1 = 0,
// { dg-note "" "" { target *-*-* } .-1 }
    V2,
    V3,
    V4 = 0,
// { dg-note "" "" { target *-*-* } .-1 }
    V5 = -2,
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    V6,
    V7,
// { dg-note "" "" { target *-*-* } .-1 }
    V8 = -3,
// { dg-note "" "" { target *-*-* } .-1 }
    V9,
// { dg-note "" "" { target *-*-* } .-1 }
}

fn main() {
}

