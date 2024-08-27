fn foo(x: u32) -> u32 {
    x * 2
}

fn bar(x: u32) -> u32 {
    x * 3
}

// original example from Issue #102608
fn foobar(n: u32) -> u32 {
    let g = if n % 2 == 0 { &foo } else { &bar };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    g(n)
}

fn main() {
    assert_eq!(foobar(7), 21);
    assert_eq!(foobar(8), 16);

    // general mismatch of fn item types
    let mut a = foo;
    a = bar;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
// { dg-error ".E0308." "" { target *-*-* } .-4 }

    // display note even when boxed
    let mut b = Box::new(foo);
    b = Box::new(bar);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }

    // suggest removing reference
    let c: fn(u32) -> u32 = &foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }

    // suggest using reference
    let d: &fn(u32) -> u32 = foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }

    // suggest casting with reference
    let e: &fn(u32) -> u32 = &foo;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }

    // OK
    let mut z: fn(u32) -> u32 = foo as fn(u32) -> u32;
    z = bar;
}

