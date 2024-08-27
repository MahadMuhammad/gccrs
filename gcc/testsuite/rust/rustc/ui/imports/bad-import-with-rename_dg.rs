mod A {
    pub type B = ();
    pub type B2 = ();
}

mod C {
    use crate::D::B as _;
// { dg-error ".E0432." "" { target *-*-* } .-1 }

    use crate::D::B2;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
}

mod D {}

fn main() {}

