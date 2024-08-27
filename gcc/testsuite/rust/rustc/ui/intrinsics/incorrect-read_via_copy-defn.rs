fn main() {
    read_via_copy();
}

extern "rust-intrinsic" fn read_via_copy() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }

