//@run-rustfix
#![allow(unused)]

struct S;
impl S {
    fn foo(&mut self) {
        let x = |v: i32| {
            self.bar();
            self.hel();
        };
        self.qux(); // { dg-error ".E0502." "" { target *-*-* } }
        x(1);
        x(3);
    }
    fn bar(&self) {}
    fn hel(&self) {}
    fn qux(&mut self) {}

    fn hello(&mut self) {
        let y = || {
            self.bar();
        };
        self.qux(); // { dg-error ".E0502." "" { target *-*-* } }
        y();
    }
}

fn main() {}

