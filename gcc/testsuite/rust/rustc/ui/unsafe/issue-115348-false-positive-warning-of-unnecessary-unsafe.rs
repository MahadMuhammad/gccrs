// Regression test for #115348.

unsafe fn uwu() {}

// Tests that the false-positive warning "unnecessary `unsafe` block"
// should not be reported, when the error "non-exhaustive patterns"
// appears.

fn foo(x: Option<u32>) {
    match x {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        Some(_) => unsafe { uwu() },
    }
}

fn main() {}

