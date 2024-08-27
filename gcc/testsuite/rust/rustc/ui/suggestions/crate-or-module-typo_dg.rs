// { dg-additional-options "-frust-edition=2018" }

use st::cell::Cell; // { dg-error ".E0433." "" { target *-*-* } }

mod bar {
    pub fn bar() { bar::baz(); } // { dg-error ".E0433." "" { target *-*-* } }

    fn baz() {}
}

use bas::bar; // { dg-error ".E0432." "" { target *-*-* } }

struct Foo {
    bar: st::cell::Cell<bool> // { dg-error ".E0433." "" { target *-*-* } }
}

fn main() {}

