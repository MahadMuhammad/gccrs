#![feature(trait_upcasting)]

trait Foo {
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

trait Bar {
    fn b(&self) -> i32 {
        20
    }

    fn w(&self) -> i32 {
        21
    }
}

trait Baz {
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
    let _: &dyn std::fmt::Debug = baz;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn Send = baz;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn Sync = baz;
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    let bar: &dyn Bar = baz;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn std::fmt::Debug = bar;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn Send = bar;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn Sync = bar;
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    let foo: &dyn Foo = baz;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn std::fmt::Debug = foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn Send = foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn Sync = foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    let foo: &dyn Foo = bar;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn std::fmt::Debug = foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn Send = foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    let _: &dyn Sync = foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

