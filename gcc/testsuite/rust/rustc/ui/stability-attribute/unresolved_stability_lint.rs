#![feature(staged_api)]
#![stable(feature = "uwu", since = "1.0.0")]

#[unstable(feature = "foo", issue = "none")]
impl Foo for () {}
// { dg-error ".E0405." "" { target *-*-* } .-1 }

fn main() {}

