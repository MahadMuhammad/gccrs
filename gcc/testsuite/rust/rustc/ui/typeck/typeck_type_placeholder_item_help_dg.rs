// This test checks that it proper item type will be suggested when
// using the `_` type placeholder.

fn test1() -> _ { Some(42) }
// { dg-error ".E0121." "" { target *-*-* } .-1 }

const TEST2: _ = 42u32;
// { dg-error ".E0121." "" { target *-*-* } .-1 }

const TEST3: _ = Some(42);
// { dg-error ".E0121." "" { target *-*-* } .-1 }

const TEST4: fn() -> _ = 42;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
// { dg-error ".E0121." "" { target *-*-* } .-2 }

trait Test5 {
    const TEST5: _ = 42;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}

struct Test6;

impl Test6 {
    const TEST6: _ = 13;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}

pub fn main() {
    let _: Option<usize> = test1(); // { dg-error ".E0308." "" { target *-*-* } }
    let _: f64 = test1(); // { dg-error ".E0308." "" { target *-*-* } }
    let _: Option<i32> = test1();
}

