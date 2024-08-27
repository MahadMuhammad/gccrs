#![feature(associated_type_defaults)]

trait Assoc {
    fn func() {}
    const CONST: u8 = 0;
    type Ty = u8;
}

trait Dyn {}

impl Assoc for dyn Dyn {}

fn main() {
    Dyn::func();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    ::Dyn::func();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    Dyn::CONST;
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    let _: Dyn::Ty; // { dg-error ".E0223." "" { target *-*-* } }
// { dg-warning ".E0223." "" { target *-*-* } .-1 }
// { dg-warning ".E0223." "" { target *-*-* } .-2 }
}

