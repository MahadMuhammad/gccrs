#![crate_type = "lib"]
#[link(kind = "static", modifiers = "+whole-archive,+bundle")]
// { dg-error ".E0459." "" { target *-*-* } .-1 }
extern  {}

