//@ aux-build: issue-118809.rs

#[macro_use]
extern crate issue_118809;

#[derive(Deserialize)] // { dg-error ".E0308." "" { target *-*-* } }
pub struct Build {
}

fn main() {}

