#![feature(trait_upcasting)]

trait Foo: PartialEq<i32> + std::fmt::Debug + Send + Sync {
    fn a(&self) -> i32 {
        10
    }

    fn z(&self) -> i32 {
        11
    }

    fn y(&self) -> i32 {
        12
    }
}

trait Bar: Foo {
    fn b(&self) -> i32 {
        20
    }

    fn w(&self) -> i32 {
        21
    }
}

trait Baz: Bar {
    fn c(&self) -> i32 {
        30
    }
}

impl Foo for i32 {
    fn a(&self) -> i32 {
        100
    }
}

impl Bar for i32 {
    fn b(&self) -> i32 {
        200
    }
}

impl Baz for i32 {
    fn c(&self) -> i32 {
        300
    }
}

fn main() {
    let baz: &dyn Baz = &1;

    let bar: &dyn Bar = baz;
    bar.c();
// { dg-error ".E0599." "" { target *-*-* } .-1 }

    let foo: &dyn Foo = baz;
    foo.b();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    foo.c();
// { dg-error ".E0599." "" { target *-*-* } .-1 }

    let foo: &dyn Foo = bar;
    foo.b();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    foo.c();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

