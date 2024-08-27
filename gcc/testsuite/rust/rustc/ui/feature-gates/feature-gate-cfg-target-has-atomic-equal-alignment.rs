fn main() {
    cfg!(target_has_atomic_equal_alignment = "8");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_equal_alignment = "16");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_equal_alignment = "32");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_equal_alignment = "64");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_equal_alignment = "128");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_equal_alignment = "ptr");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

