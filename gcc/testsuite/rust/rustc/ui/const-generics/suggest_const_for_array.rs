#![crate_type = "lib"]

fn example<const N: usize>() {}

fn other() {
    example::<[usize; 3]>();
// { dg-error ".E0747." "" { target *-*-* } .-1 }
    example::<[usize; 4 + 5]>();
// { dg-error ".E0747." "" { target *-*-* } .-1 }
}

