// Regression test for https://github.com/rust-lang/rust/issues/122581
// This used to ICE, because the union was unsized and the pointer casting code
// assumed that non-struct ADTs must be sized.

union Union {
    val: std::mem::ManuallyDrop<[u8]>,
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn cast(ptr: *const ()) -> *const Union {
    ptr as _
}

fn main() {}

