use std::ops::Deref;

struct Foo {
    v: Vec<u32>,
}

struct Bar {
    v: Vec<u32>,
}

impl Deref for Bar {
    type Target = Vec<u32>;

    fn deref(&self) -> &Self::Target {
        &self.v
    }
}

fn f(foo: &Foo) {
    match foo {
        Foo { v: [1, 2] } => {}
// { dg-error ".E0529." "" { target *-*-* } .-1 }
        _ => {}
    }
}

fn bar(bar: &Bar) {
    match bar {
        Bar { v: [1, 2] } => {}
// { dg-error ".E0529." "" { target *-*-* } .-1 }
        _ => {}
    }
}

fn main() {}

