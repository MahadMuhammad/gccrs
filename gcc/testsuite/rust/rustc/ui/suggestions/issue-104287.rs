// The purpose of this test is not to validate the output of the compiler.
// Instead, it ensures the suggestion is generated without performing an arithmetic overflow.

struct S;
impl S {
    fn foo(&self) {}
}
fn main() {
    let x = S;
    foo::<()>(x);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
}

