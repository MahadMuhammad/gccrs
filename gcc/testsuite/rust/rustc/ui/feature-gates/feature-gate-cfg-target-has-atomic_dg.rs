fn main() {
    cfg!(target_has_atomic_load_store = "8");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_load_store = "16");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_load_store = "32");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_load_store = "64");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_load_store = "128");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    cfg!(target_has_atomic_load_store = "ptr");
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

