trait Foo {
    type Bar<T>;
}

fn bar(x: &dyn Foo) {} // { dg-error ".E0038." "" { target *-*-* } }
// { dg-error ".E0038." "" { target *-*-* } .-1 }
// { dg-error ".E0038." "" { target *-*-* } .-2 }
// { dg-error ".E0038." "" { target *-*-* } .-3 }

fn main() {}

