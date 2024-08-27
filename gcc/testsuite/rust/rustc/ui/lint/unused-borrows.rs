#![deny(unused_must_use)]

fn foo(_: i32) -> bool { todo!() }

fn bar() -> &'static i32 {
    &42;
// { dg-error "" "" { target *-*-* } .-1 }

    &mut foo(42);
// { dg-error "" "" { target *-*-* } .-1 }

    &&42;
// { dg-error "" "" { target *-*-* } .-1 }

    &&mut 42;
// { dg-error "" "" { target *-*-* } .-1 }

    &mut &42;
// { dg-error "" "" { target *-*-* } .-1 }

    let _result = foo(4)
        && foo(2); // Misplaced semi-colon (perhaps due to reordering of lines)
    && foo(42);
// { dg-error "" "" { target *-*-* } .-1 }

    let _ = &42; // ok

    &42 // ok
}

fn main() {
    let _ = bar();
}

