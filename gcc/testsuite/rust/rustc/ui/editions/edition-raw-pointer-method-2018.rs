// { dg-additional-options "-frust-edition=2018" }

// tests that editions work with the tyvar warning-turned-error

#[deny(warnings)]
fn main() {
    let x = 0;
    let y = &x as *const _;
// { dg-error ".E0282." "" { target *-*-* } .-1 }
    let _ = y.is_null();
}

