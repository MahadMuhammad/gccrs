// Issue #90528: provide helpful suggestions when a trait bound is unsatisfied
// due to a missed unsizing coercion.
//
// This test exercises array variables and a trait implemented on mutable slices.

trait Write {}

impl Write for &mut [u8] {}

fn wants_write(_: impl Write) {}

fn main() {
    let mut x = [0u8];
    wants_write(x);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    wants_write(&mut x);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    wants_write(&mut x[..]);

    let x = &mut [0u8];
    wants_write(x);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    wants_write(*x);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    wants_write(&mut x[..]);
}

