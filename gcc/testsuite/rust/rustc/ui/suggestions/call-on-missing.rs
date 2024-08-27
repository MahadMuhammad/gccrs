struct Foo { i: i32 }

impl Foo {
    fn bar(&self) {}
}

fn foo() -> Foo {
    Foo { i: 1 }
}

fn main() {
    foo.bar();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }

    foo.i;
// { dg-error ".E0609." "" { target *-*-* } .-1 }
// { help ".E0609." "" { target *-*-* } .-2 }

    let callable = Box::new(|| Foo { i: 1 }) as Box<dyn Fn() -> Foo>;

    callable.bar();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }

    callable.i;
// { dg-error ".E0609." "" { target *-*-* } .-1 }
// { help ".E0609." "" { target *-*-* } .-2 }
}

fn type_param<T: Fn() -> Foo>(t: T) {
    t.bar();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }

    t.i;
// { dg-error ".E0609." "" { target *-*-* } .-1 }
// { help ".E0609." "" { target *-*-* } .-2 }
}

