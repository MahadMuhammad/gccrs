// Test that we emit an error if we cannot properly infer a constant.

// taken from https://github.com/rust-lang/rust/issues/70507#issuecomment-615268893
struct Foo;
impl Foo {
    fn foo<const A: usize, const B: usize>(self) {}
}
fn main() {
    Foo.foo();
// { dg-error ".E0284." "" { target *-*-* } .-1 }
// { dg-error ".E0284." "" { target *-*-* } .-2 }
}

