fn foo() {
// { help "" "" { target *-*-* } .-1 }
    |x: &i32| 1i32
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn bar(i: impl Sized) {
// { help "" "" { target *-*-* } .-1 }
    || i
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

