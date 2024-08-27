//@ run-rustfix
//@ rustfix-only-machine-applicable
//@ build-pass (FIXME(62277): should be check-pass)
//@ aux-build:fancy-panic.rs

extern crate fancy_panic;

const C: &str = "abc {}";
static S: &str = "{bla}";

#[allow(unreachable_code)]
fn main() {
    panic!("here's a brace: {"); // { dg-warning "" "" { target *-*-* } }
    unreachable!("here's a brace: {"); // { dg-warning "" "" { target *-*-* } }
    std::panic!("another one: }"); // { dg-warning "" "" { target *-*-* } }
    core::panic!("Hello {}"); // { dg-warning "" "" { target *-*-* } }
    assert!(false, "{:03x} {test} bla");
// { dg-warning "" "" { target *-*-* } .-1 }
    assert!(false, S);
// { dg-warning "" "" { target *-*-* } .-1 }
    assert!(false, 123);
// { dg-warning "" "" { target *-*-* } .-1 }
    assert!(false, Some(123));
// { dg-warning "" "" { target *-*-* } .-1 }
    debug_assert!(false, "{{}} bla"); // { dg-warning "" "" { target *-*-* } }
    panic!(C); // { dg-warning "" "" { target *-*-* } }
    panic!(S); // { dg-warning "" "" { target *-*-* } }
    unreachable!(S); // { dg-warning "" "" { target *-*-* } }
    unreachable!(S); // { dg-warning "" "" { target *-*-* } }
    std::panic!(123); // { dg-warning "" "" { target *-*-* } }
    core::panic!(&*"abc"); // { dg-warning "" "" { target *-*-* } }
    panic!(Some(123)); // { dg-warning "" "" { target *-*-* } }
    panic!(concat!("{", "}")); // { dg-warning "" "" { target *-*-* } }
    panic!(concat!("{", "{")); // { dg-warning "" "" { target *-*-* } }

    fancy_panic::fancy_panic!("test {} 123");
// { dg-warning "" "" { target *-*-* } .-1 }

    fancy_panic::fancy_panic!(); // OK
    fancy_panic::fancy_panic!(S); // OK

    macro_rules! a {
        () => { 123 };
    }

    panic!(a!()); // { dg-warning "" "" { target *-*-* } }
    unreachable!(a!()); // { dg-warning "" "" { target *-*-* } }

    panic!(format!("{}", 1)); // { dg-warning "" "" { target *-*-* } }
    unreachable!(format!("{}", 1)); // { dg-warning "" "" { target *-*-* } }
    assert!(false, format!("{}", 1)); // { dg-warning "" "" { target *-*-* } }
    debug_assert!(false, format!("{}", 1)); // { dg-warning "" "" { target *-*-* } }

    panic![123]; // { dg-warning "" "" { target *-*-* } }
    panic!{123}; // { dg-warning "" "" { target *-*-* } }

    // Check that the lint only triggers for std::panic and core::panic,
    // not any panic macro:
    macro_rules! panic {
        ($e:expr) => ();
    }
    panic!("{}"); // OK
    panic!(S); // OK

    a(1);
    b(1);
    c(1);
    d(1);
}

fn a<T: Send + 'static>(v: T) {
    panic!(v); // { dg-warning "" "" { target *-*-* } }
    assert!(false, v); // { dg-warning "" "" { target *-*-* } }
}

fn b<T: std::fmt::Debug + Send + 'static>(v: T) {
    panic!(v); // { dg-warning "" "" { target *-*-* } }
    assert!(false, v); // { dg-warning "" "" { target *-*-* } }
}

fn c<T: std::fmt::Display + Send + 'static>(v: T) {
    panic!(v); // { dg-warning "" "" { target *-*-* } }
    assert!(false, v); // { dg-warning "" "" { target *-*-* } }
}

fn d<T: std::fmt::Display + std::fmt::Debug + Send + 'static>(v: T) {
    panic!(v); // { dg-warning "" "" { target *-*-* } }
    assert!(false, v); // { dg-warning "" "" { target *-*-* } }
}

