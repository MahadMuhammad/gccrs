trait Foo {
    type Type;

    fn foo();
    fn bar();
    fn qux();
}

struct A;

impl Foo for A {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
    type Typ = ();
// { dg-error ".E0437." "" { target *-*-* } .-1 }
// { help ".E0437." "" { target *-*-* } .-2 }

    fn fooo() {}
// { dg-error ".E0407." "" { target *-*-* } .-1 }
// { help ".E0407." "" { target *-*-* } .-2 }

    fn barr() {}
// { dg-error ".E0407." "" { target *-*-* } .-1 }
// { help ".E0407." "" { target *-*-* } .-2 }

    fn quux() {}
// { dg-error ".E0407." "" { target *-*-* } .-1 }
// { help ".E0407." "" { target *-*-* } .-2 }
}
// { help "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }

trait Bar {
    const Const: i32;
}

struct B;

impl Bar for B {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
    const Cnst: i32 = 0;
// { dg-error ".E0438." "" { target *-*-* } .-1 }
// { help ".E0438." "" { target *-*-* } .-2 }
}
// { help "" "" { target *-*-* } .-1 }

fn main() {}

