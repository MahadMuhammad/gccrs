trait Trait {}

struct A;
struct B;
struct C;

impl Trait for &A {}
impl Trait for &mut A {}

impl Trait for &B {}

impl Trait for &mut C {}

fn foo<X: Trait>(_: X) {}

fn main() {
    let a = A;
    let b = B;
    let c = C;
    foo(a); // { dg-error ".E0277." "" { target *-*-* } }
    foo(b); // { dg-error ".E0277." "" { target *-*-* } }
    foo(c); // { dg-error ".E0277." "" { target *-*-* } }
}

