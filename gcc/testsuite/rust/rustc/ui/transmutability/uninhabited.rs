#![crate_type = "lib"]
#![feature(transmutability)]
#![allow(dead_code, incomplete_features, non_camel_case_types)]

mod assert {
    use std::mem::{Assume, TransmuteFrom};

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: TransmuteFrom<Src, {
            Assume {
                alignment: true,
                lifetimes: true,
                safety: true,
                validity: true,
            }
        }>
    {}
}

fn void() {
    enum Void {}

    // This transmutation is vacuously acceptable; since one cannot construct a
    // `Void`, unsoundness cannot directly arise from transmuting a void into
    // anything else.
    assert::is_maybe_transmutable::<Void, u128>();

    assert::is_maybe_transmutable::<(), Void>(); // { dg-error ".E0277." "" { target *-*-* } }
}

// Non-ZST uninhabited types are, nonetheless, uninhabited.
fn yawning_void_struct() {
    enum Void {}

    struct YawningVoid(Void, u128);

    const _: () = {
        assert!(std::mem::size_of::<YawningVoid>() == std::mem::size_of::<u128>());
        // Just to be sure the above constant actually evaluated:
        assert!(false); // { dg-error ".E0080." "" { target *-*-* } }
    };

    // This transmutation is vacuously acceptable; since one cannot construct a
    // `Void`, unsoundness cannot directly arise from transmuting a void into
    // anything else.
    assert::is_maybe_transmutable::<YawningVoid, u128>();

    assert::is_maybe_transmutable::<(), Void>(); // { dg-error ".E0277." "" { target *-*-* } }
}

// Non-ZST uninhabited types are, nonetheless, uninhabited.
fn yawning_void_enum() {
    enum Void {}

    enum YawningVoid {
        A(Void, u128),
    }

    const _: () = {
        assert!(std::mem::size_of::<YawningVoid>() == std::mem::size_of::<u128>());
        // Just to be sure the above constant actually evaluated:
        assert!(false); // { dg-error ".E0080." "" { target *-*-* } }
    };

    // This transmutation is vacuously acceptable; since one cannot construct a
    // `Void`, unsoundness cannot directly arise from transmuting a void into
    // anything else.
    assert::is_maybe_transmutable::<YawningVoid, u128>();

    assert::is_maybe_transmutable::<(), Void>(); // { dg-error ".E0277." "" { target *-*-* } }
}

// References to uninhabited types are, logically, uninhabited, but for layout
// purposes are not ZSTs, and aren't treated as uninhabited when they appear in
// enum variants.
fn distant_void() {
    enum Void {}

    enum DistantVoid {
        A(&'static Void)
    }

    const _: () = {
        assert!(std::mem::size_of::<DistantVoid>() == std::mem::size_of::<usize>());
        // Just to be sure the above constant actually evaluated:
        assert!(false); // { dg-error ".E0080." "" { target *-*-* } }
    };

    assert::is_maybe_transmutable::<DistantVoid, ()>();
    assert::is_maybe_transmutable::<DistantVoid, &'static Void>();
    assert::is_maybe_transmutable::<u128, DistantVoid>(); // { dg-error ".E0277." "" { target *-*-* } }
}

