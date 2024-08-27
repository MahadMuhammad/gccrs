fn foo<T: Fn()>(t: T) {
    t(1i32);
// { dg-error ".E0057." "" { target *-*-* } .-1 }
}

fn bar(t: impl Fn()) {
    t(1i32);
// { dg-error ".E0057." "" { target *-*-* } .-1 }
}

fn baz() -> impl Fn() {
    || {}
}

fn baz2() {
    baz()(1i32)
// { dg-error ".E0057." "" { target *-*-* } .-1 }
}

fn qux() {
    let x = || {};
    x(1i32);
// { dg-error ".E0057." "" { target *-*-* } .-1 }
}

fn main() {}

