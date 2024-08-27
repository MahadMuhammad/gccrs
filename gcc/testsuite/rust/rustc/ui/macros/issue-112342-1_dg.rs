// same as #95267, ignore doc comment although it's a bug.

macro_rules! m1 {
    (
        $(
            ///
        )*
// { dg-error "" "" { target *-*-* } .-3 }
    ) => {};
}

m1! {}

macro_rules! m2 {
    (
        $(
            ///
        )+
// { dg-error "" "" { target *-*-* } .-3 }
    ) => {};
}

m2! {}

macro_rules! m3 {
    (
        $(
            ///
        )?
// { dg-error "" "" { target *-*-* } .-3 }
    ) => {};
}

m3! {}


macro_rules! m4 {
    (
        $(
            ///
            ///
        )*
// { dg-error "" "" { target *-*-* } .-4 }
    ) => {};
}

m4! {}

fn main() {}

