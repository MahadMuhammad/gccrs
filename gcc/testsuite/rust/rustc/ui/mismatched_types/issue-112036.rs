struct Foo;

impl Drop for Foo {
    fn drop(self) {} // { dg-error ".E0053." "" { target *-*-* } }
}

fn main() {}

