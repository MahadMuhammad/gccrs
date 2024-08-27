// Test that regionck suggestions in a provided method of a trait
// don't ICE

trait Foo<'a> {
    fn foo(&'a self);
    fn bar(&self) {
        self.foo();
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

fn main() {}

