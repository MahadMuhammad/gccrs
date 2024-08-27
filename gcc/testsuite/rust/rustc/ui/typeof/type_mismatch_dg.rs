// Test that using typeof results in the correct type mismatch errors instead of always assuming
// `usize`, in addition to the pre-existing "typeof is reserved and unimplemented" error
fn main() {
    const a: u8 = 1;
    let b: typeof(a) = 1i8;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
}

