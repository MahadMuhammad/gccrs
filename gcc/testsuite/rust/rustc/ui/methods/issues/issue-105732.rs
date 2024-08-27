#![feature(auto_traits)]

auto trait Foo {
    fn g(&self); // { dg-error ".E0380." "" { target *-*-* } }
}

trait Bar {
    fn f(&self) {
        // issue #105788
        self.g(); // { dg-error ".E0599." "" { target *-*-* } }
    }
}

fn main() {}

