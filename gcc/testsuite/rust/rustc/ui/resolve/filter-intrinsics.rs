fn main() {
    // Should suggest only `std::mem::transmute`
    let _ = transmute::<usize>();
// { dg-error ".E0425." "" { target *-*-* } .-1 }

    // Should suggest `std::intrinsics::fabsf64`,
    // since there is no non-intrinsic to suggest.
    let _ = fabsf64(1.0);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

