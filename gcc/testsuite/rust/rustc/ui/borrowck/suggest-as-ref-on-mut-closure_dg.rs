// This is not exactly right, yet.

// Ideally we should be suggesting `as_mut` for the first case,
// and suggesting to change `as_ref` to `as_mut` in the second.

fn x(cb: &mut Option<&mut dyn FnMut()>) {
    cb.map(|cb| cb());
// { dg-error ".E0507." "" { target *-*-* } .-1 }
}

fn x2(cb: &mut Option<&mut dyn FnMut()>) {
    cb.as_ref().map(|cb| cb());
// { dg-error ".E0596." "" { target *-*-* } .-1 }
}

fn main() {}

