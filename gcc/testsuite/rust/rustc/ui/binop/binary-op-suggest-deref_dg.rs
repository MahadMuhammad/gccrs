#![allow(dead_code)]

fn foo() {
    // Issue #52544
    let i: &i64 = &1;
    if i < 0 {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn bar() {
    // Issue #40660
    let foo = &&0;

    // Dereference LHS
    _ = foo == 0;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = foo == &0;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = &&&&foo == 0;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = *foo == 0;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = &&foo == &&0;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = &Box::new(42) == 42;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = &Box::new(&Box::new(&42)) == 42;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    // Dereference RHS
    _ = 0 == foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = &0 == foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = 0 == &&&&foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = 0 == *foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = &&0 == &&foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    // Dereference both sides
    _ = &Box::new(Box::new(42)) == &foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = &Box::new(42) == &foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = &Box::new(Box::new(Box::new(Box::new(42)))) == &foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = &foo == &Box::new(Box::new(Box::new(Box::new(42))));
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    // Don't suggest dereferencing the LHS; suggest boxing the RHS instead
    _ = Box::new(42) == 42;
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    // Don't suggest dereferencing with types that can't be compared
    struct Foo;
    _ = &&0 == Foo;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = Foo == &&0;
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

fn baz() {
    // Issue #44695
    let owned = "foo".to_owned();
    let string_ref = &owned;
    let partial = "foobar";
    _ = string_ref == partial[..3];
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    _ = partial[..3] == string_ref;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn qux() {
    // Issue #119352
    const FOO: i32 = 42;
    let _ = FOO & (*"Sized".to_string().into_boxed_str());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

fn main() {}

