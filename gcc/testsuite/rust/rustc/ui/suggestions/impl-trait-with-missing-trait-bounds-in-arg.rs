//@ run-rustfix

trait Foo {}

trait Bar {
    fn hello(&self) {}
}

struct S;

impl Foo for S {}
impl Bar for S {}

fn test(foo: impl Foo) {
    foo.hello(); // { dg-error ".E0599." "" { target *-*-* } }
}

trait Trait {
    fn method(&self) {}
}

impl Trait for fn() {}

#[allow(dead_code)]
fn test2(f: impl Fn() -> dyn std::fmt::Debug) {
    f.method(); // { dg-error ".E0599." "" { target *-*-* } }
}

fn main() {
    test(S);
}

