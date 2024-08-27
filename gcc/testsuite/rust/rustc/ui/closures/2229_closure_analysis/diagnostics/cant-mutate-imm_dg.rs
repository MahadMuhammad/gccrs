// { dg-additional-options "-frust-edition=2021" }

// Ensure that diagnostics for mutability error (because the root variable
// isn't mutable) work with `capture_disjoint_fields` enabled.

fn mut_error_struct() {
    let x = (10, 10);
    let y = (x, 10);
    let z = (y, 10);

    let mut c = || {
        z.0.0.0 = 20;
// { dg-error ".E0594." "" { target *-*-* } .-1 }
    };

    c();
}

fn mut_error_box() {
    let x = (10, 10);
    let bx = Box::new(x);

    let mut c = || {
        bx.0 = 20;
// { dg-error ".E0594." "" { target *-*-* } .-1 }
    };

    c();
}

fn main() {
    mut_error_struct();
    mut_error_box();
}

