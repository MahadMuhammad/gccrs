fn main() {
    transmute(); // does not ICE
}

extern "rust-intrinsic" fn transmute() {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }

