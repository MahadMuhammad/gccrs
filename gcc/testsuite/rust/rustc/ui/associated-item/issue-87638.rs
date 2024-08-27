//@ run-rustfix

trait Trait {
    const FOO: usize;

    type Target;
}

struct S;

impl Trait for S {
    const FOO: usize = 0;
    type Target = ();
}

fn main() {
    let _: <S as Trait>::Output; // { dg-error ".E0576." "" { target *-*-* } }
// { help ".E0576." "" { target *-*-* } .-1 }

    let _ = <S as Trait>::BAR; // { dg-error ".E0576." "" { target *-*-* } }
// { help ".E0576." "" { target *-*-* } .-1 }
}

