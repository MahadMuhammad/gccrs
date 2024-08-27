pub trait TraitWAssocConst {
    const A:   usize;
}
pub struct Demo {}

impl TraitWAssocConst for impl Demo { // { dg-error ".E0562." "" { target *-*-* } }
// { dg-error ".E0562." "" { target *-*-* } .-1 }
    pubconst A: str = 32; // { dg-error "" "" { target *-*-* } }
}

fn foo<A: TraitWAssocConst<A=32>>() { // { dg-error ".E0658." "" { target *-*-* } }
    foo::<Demo>()();
// { dg-error ".E0618." "" { target *-*-* } .-1 }
// { dg-error ".E0618." "" { target *-*-* } .-2 }
}

fn main<A: TraitWAssocConst<A=32>>() {
// { dg-error ".E0131." "" { target *-*-* } .-1 }
// { dg-error ".E0131." "" { target *-*-* } .-2 }
    foo::<Demo>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

