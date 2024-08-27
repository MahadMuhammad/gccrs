struct Struct<T>(T);

impl<T> Struct<T> {
    const CONST: fn() = || {
        struct _Obligation where T:; // { dg-error ".E0401." "" { target *-*-* } }
    };
}

fn main() {}

