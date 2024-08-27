//! This test used to actually start evaluating the static even though
//! there were errors in typeck.
//! issue: rust-lang/rust#123153

pub struct Foo {
    pub version: str,
}

pub struct Bar {
    pub ok: &'static [&'static Bar],
    pub bad: &'static Foo,
}

pub static WITH_ERROR: Foo = Foo { version: 0 };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }

pub static USE_WITH_ERROR: Bar = Bar { ok: &[], bad: &WITH_ERROR };

fn main() {}

