// Test to ensure that there is no ICE when normalizing a projection
// which is invalid (from <https://github.com/rust-lang/rust/pull/106938>).

#![crate_type = "lib"]

pub trait Identity {
    type Identity;
}

pub type Foo = u8;

pub union Bar {
    a:  <Foo as Identity>::Identity, // { dg-error ".E0277." "" { target *-*-* } }
    b: u8,
}

