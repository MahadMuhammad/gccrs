// This test ensures that if implementation on projections is supported,
// it doesn't end in very weird cycle error.

#![crate_type = "lib"]

pub trait Identity {
    type Identity: ?Sized;
}

impl<T: ?Sized> Identity for T {
    type Identity = Self;
}

pub struct I8<const F: i8>;

impl <I8<{i8::MIN}> as Identity>::Identity {
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-error ".E0599." "" { target *-*-* } .-2 }
    pub fn foo(&self) {}
}

