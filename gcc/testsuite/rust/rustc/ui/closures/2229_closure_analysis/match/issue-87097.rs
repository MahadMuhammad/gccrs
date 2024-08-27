//@ run-pass
// { dg-additional-options "-frust-edition=2021" }

enum Variant {
    A,
    B, // { dg-warning "" "" { target *-*-* } }
}

struct A {
    field: Variant,
}

fn discriminant_is_a_ref() {
    let here = A { field: Variant::A };
    let out_ref = &here.field;

    || match out_ref { // { dg-warning "" "" { target *-*-* } }
        Variant::A => (),
        Variant::B => (),
    };
}

fn discriminant_is_a_field() {
    let here = A { field: Variant::A };

    || match here.field { // { dg-warning "" "" { target *-*-* } }
        Variant::A => (),
        Variant::B => (),
    };
}

fn main() {
    discriminant_is_a_ref();
    discriminant_is_a_field();
}

