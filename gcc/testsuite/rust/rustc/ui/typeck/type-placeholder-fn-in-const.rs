struct MyStruct;

trait Test {
    const TEST: fn() -> _;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-error ".E0121." "" { target *-*-* } .-2 }
}

impl Test for MyStruct {
    const TEST: fn() -> _ = 42;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-error ".E0121." "" { target *-*-* } .-2 }
}

fn main() {}

