struct S {}

impl S {
    fn f(self: &S::x) {} // { dg-error ".E0223." "" { target *-*-* } }
}

fn main() {}

