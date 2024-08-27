struct Foo {}

impl Foo {
    pub fn foo(&mut self) {
        || bar(&mut self);
// { dg-error ".E0596." "" { target *-*-* } .-1 }
    }

    pub fn baz(&self) {
        || bar(&mut self);
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { dg-error ".E0596." "" { target *-*-* } .-2 }
    }

    pub fn qux(mut self) {
        || bar(&mut self);
        // OK
    }

    pub fn quux(self) {
        || bar(&mut self);
// { dg-error ".E0596." "" { target *-*-* } .-1 }
    }
}

fn bar(_: &mut Foo) {}

fn main() {}

