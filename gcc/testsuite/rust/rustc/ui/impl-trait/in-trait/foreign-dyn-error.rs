//@ aux-build: rpitit.rs

extern crate rpitit;

fn main() {
    let _: &dyn rpitit::Foo = todo!();
// { dg-error ".E0038." "" { target *-*-* } .-1 }
}

