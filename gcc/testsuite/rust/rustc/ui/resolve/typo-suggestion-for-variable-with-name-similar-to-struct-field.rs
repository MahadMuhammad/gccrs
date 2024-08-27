struct A {
    config: String,
}

impl A {
    fn new(cofig: String) -> Self {
        Self { config } // { dg-error ".E0425." "" { target *-*-* } }
    }

    fn do_something(cofig: String) {
        println!("{config}"); // { dg-error ".E0425." "" { target *-*-* } }
    }

    fn self_is_available(self, cofig: String) {
        println!("{config}"); // { dg-error ".E0425." "" { target *-*-* } }
    }
}

trait B {
    const BAR: u32 = 3;
    type Baz;
    fn bar(&self);
    fn baz(&self) {}
    fn bah() {}
}

impl B for Box<isize> {
    type Baz = String;
    fn bar(&self) {
        // let baz = 3;
        baz();
// { dg-error ".E0425." "" { target *-*-* } .-1 }
        bah;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
        BAR;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
        let foo: Baz = "".to_string();
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    }
}

fn ba() {}
const BARR: u32 = 3;
type Bar = String;

fn main() {}

