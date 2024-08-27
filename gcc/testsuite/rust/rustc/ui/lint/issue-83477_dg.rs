//@ compile-flags: -Zunstable-options
//@ check-pass
#![warn(rustc::internal)]

#[allow(rustc::foo::bar::default_hash_types)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { suggestion "" "" { target *-*-* } .-3 }
#[allow(rustc::foo::default_hash_types)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { suggestion "" "" { target *-*-* } .-3 }
fn main() {
    let _ = std::collections::HashMap::<String, String>::new();
// { dg-warning "" "" { target *-*-* } .-1 }
}

