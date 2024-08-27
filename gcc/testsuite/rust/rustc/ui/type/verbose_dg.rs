//@ revisions:verbose normal
//@ [verbose]compile-flags:--verbose
#![crate_type = "lib"]

struct Foo<T, U> { x: T, y: U }
fn bar() {
    let _: Foo<u32, i32> = Foo::<i32, i32> { x: 0, y: 0 };
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
}

