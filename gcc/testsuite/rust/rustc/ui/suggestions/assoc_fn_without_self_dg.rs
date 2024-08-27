fn main() {}

struct S;

impl S {
    fn foo() {}

    fn bar(&self) {}

    fn baz(a: u8, b: u8) {}

    fn b() {
        fn c() {
            foo(); // { dg-error ".E0425." "" { target *-*-* } }
        }
        foo(); // { dg-error ".E0425." "" { target *-*-* } }
        bar(); // { dg-error ".E0425." "" { target *-*-* } }
        baz(2, 3); // { dg-error ".E0425." "" { target *-*-* } }
    }
    fn d(&self) {
        fn c() {
            foo(); // { dg-error ".E0425." "" { target *-*-* } }
        }
        foo(); // { dg-error ".E0425." "" { target *-*-* } }
        bar(); // { dg-error ".E0425." "" { target *-*-* } }
        baz(2, 3); // { dg-error ".E0425." "" { target *-*-* } }
    }
}

