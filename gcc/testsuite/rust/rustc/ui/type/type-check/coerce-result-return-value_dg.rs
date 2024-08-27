//@ run-rustfix
struct A;
struct B;
impl From<A> for B {
    fn from(_: A) -> Self { B }
}
fn foo1(x: Result<(), A>) -> Result<(), B> {
    x // { dg-error ".E0308." "" { target *-*-* } }
}
fn foo2(x: Result<(), A>) -> Result<(), B> {
    return x; // { dg-error ".E0308." "" { target *-*-* } }
}
fn foo3(x: Result<(), A>) -> Result<(), B> {
    if true {
        x // { dg-error ".E0308." "" { target *-*-* } }
    } else {
        x // { dg-error ".E0308." "" { target *-*-* } }
    }
}
fn main() {
    let _ = foo1(Ok(()));
    let _ = foo2(Ok(()));
    let _ = foo3(Ok(()));
}

