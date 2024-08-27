#![allow(warnings)]

fn issue_117846_repro() {
    let (a, _) = if true {
        produce()
    } else {
        (Vec::new(), &[]) // { dg-error ".E0308." "" { target *-*-* } }
    };

    accept(&a);
}

struct Foo;
struct Bar;

fn produce() -> (Vec<Foo>, &'static [Bar]) {
    todo!()
}

fn accept(c: &[Foo]) {}

fn main() {}

