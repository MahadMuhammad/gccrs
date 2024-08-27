struct Foo<'a>(&'a str);
struct Buzz<'a, 'b>(&'a str, &'b str);
struct Qux<'a, T>(&'a T);
struct Quux<T>(T);

enum Bar {
    A,
    B,
    C,
}

struct Baz<'a, 'b, 'c> {
    buzz: Buzz<'a>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    bar: Bar<'a>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    foo2: Foo<'a, 'b, 'c>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    qux1: Qux<'a, 'b, i32>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    qux2: Qux<'a, i32, 'b>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    qux3: Qux<'a, 'b, 'c, i32>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    qux4: Qux<'a, i32, 'b, 'c>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    qux5: Qux<'a, 'b, i32, 'c>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }

    quux: Quux<'a, i32, 'b>,
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
}

pub trait T {
    type A;
    type B;
}

fn trait_bound_generic<I: T<u8, u16>>(_i: I) {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
// { help ".E0107." "" { target *-*-* } .-2 }
}

fn main() {}

