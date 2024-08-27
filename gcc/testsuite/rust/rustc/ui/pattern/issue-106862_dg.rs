//@ run-rustfix

#![allow(unused)]

use Foo::{FooB, FooA};

enum Foo {
    FooA { opt_x: Option<i32>, y: i32 },
    FooB { x: i32, y: i32 }
}

fn main() {
    let f = FooB { x: 3, y: 4 };

    match f {
        FooB(a, b) => println!("{} {}", a, b),
// { dg-error ".E0532." "" { target *-*-* } .-1 }
        _ => (),
    }

    match f {
        FooB(x, y) => println!("{} {}", x, y),
// { dg-error ".E0532." "" { target *-*-* } .-1 }
        _ => (),
    }

    match f {
        FooA(Some(x), y) => println!("{} {}", x, y),
// { dg-error ".E0532." "" { target *-*-* } .-1 }
        _ => (),
    }

    match f {
        FooB(a, _, _) => println!("{}", a),
// { dg-error ".E0532." "" { target *-*-* } .-1 }
        _ => (),
    }

    match f {
        FooB() => (),
// { dg-error ".E0532." "" { target *-*-* } .-1 }
        _ => (),
    }
}

