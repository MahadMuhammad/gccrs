//@ revisions: current next
//@[next] compile-flags: -Znext-solver

trait MyDebug {
    fn my_debug(&self);
}

impl MyDebug for &() {
    fn my_debug(&self) {}
}

fn my_foo() -> impl std::fmt::Debug {
    if false {
        let x = &my_foo();
// { dg-error "" "" { target *-*-* } .-1 }
        x.my_debug();
// { dg-error "" "" { target *-*-* } .-1 }
    }
    ()
}

fn main() {}

