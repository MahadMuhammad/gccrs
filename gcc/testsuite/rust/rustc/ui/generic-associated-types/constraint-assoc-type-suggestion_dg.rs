// Test that correct syntax is used in suggestion to constrain associated type

trait X {
    type Y<T>;
}

fn f<T: X>(a: T::Y<i32>) {
// { help "" "" { target *-*-* } .-1 }
// { suggestion "" "" { target *-*-* } .-2 }
    let b: Vec<i32> = a;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

