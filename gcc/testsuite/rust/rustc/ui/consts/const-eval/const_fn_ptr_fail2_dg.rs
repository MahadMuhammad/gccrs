//@ compile-flags: -Zunleash-the-miri-inside-of-you

fn double(x: usize) -> usize {
    x * 2
}
const X: fn(usize) -> usize = double;

const fn bar(x: fn(usize) -> usize, y: usize) -> usize {
    x(y)
// { dg-error ".E0080." "" { target *-*-* } .-1 }
// { dg-error ".E0080." "" { target *-*-* } .-2 }
}

const Y: usize = bar(X, 2); // FIXME: should fail to typeck someday
const Z: usize = bar(double, 2); // FIXME: should fail to typeck someday

fn main() {
    assert_eq!(Y, 4);
    assert_eq!(Z, 4);
}

