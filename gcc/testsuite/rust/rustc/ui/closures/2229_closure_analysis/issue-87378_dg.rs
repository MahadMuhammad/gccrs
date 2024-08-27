#![feature(rustc_attrs)]

// { dg-additional-options "-frust-edition=2021" }

// Test that any precise capture on a union is truncated because it's unsafe to do so.

union Union {
    value: u64,
}

fn main() {
    let u = Union { value: 42 };

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
       unsafe { u.value }
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();
}

