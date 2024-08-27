//@ run-rustfix
pub mod a {
    pub struct A(pub(self)String);
}

mod b {
    use crate::a::A;
    pub fn x() {
        A("".into()); // { dg-error ".E0423." "" { target *-*-* } }
    }
}
fn main() {
    a::A("a".into()); // { dg-error ".E0603." "" { target *-*-* } }
    b::x();
}

