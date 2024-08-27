trait Zero {
    const ZERO: Self;
}

impl Zero for String {
    const ZERO: Self = String::new();
}

fn foo() {
     match String::new() {
        Zero::ZERO ..= Zero::ZERO => {},
// { dg-error ".E0029." "" { target *-*-* } .-1 }
        _ => {},
    }
}

fn bar() {
    match Zero::ZERO {
        Zero::ZERO ..= Zero::ZERO => {},
// { dg-error ".E0282." "" { target *-*-* } .-1 }
        _ => {},
    }
}

fn main() {
    foo();
    bar();
}

