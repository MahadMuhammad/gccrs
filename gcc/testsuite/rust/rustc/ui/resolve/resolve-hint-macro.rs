//@ run-rustfix
fn main() {
    assert_eq(1, 1);
// { dg-error ".E0423." "" { target *-*-* } .-1 }
    assert_eq { 1, 1 };
// { dg-error ".E0574." "" { target *-*-* } .-1 }
// { dg-error ".E0574." "" { target *-*-* } .-2 }
// { dg-error ".E0574." "" { target *-*-* } .-3 }
    assert[true];
// { dg-error ".E0423." "" { target *-*-* } .-1 }
}

