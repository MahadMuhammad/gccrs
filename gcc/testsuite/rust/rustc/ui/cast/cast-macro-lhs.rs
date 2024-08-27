// Test to make sure we suggest "consider casting" on the right span

macro_rules! foo {
    () => { 0 }
}

fn main() {
    let x = foo!() as *const [u8];
// { dg-error ".E0606." "" { target *-*-* } .-1 }
// { dg-note ".E0606." "" { target *-*-* } .-2 }
// { dg-note ".E0606." "" { target *-*-* } .-3 }
}

