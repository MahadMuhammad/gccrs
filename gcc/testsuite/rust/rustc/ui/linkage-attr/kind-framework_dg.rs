//@ ignore-apple this is supposed to succeed on Apple platforms (though it won't necessarily link)

#[link(name = "foo", kind = "framework")]
extern "C" {}
// { dg-error ".E0455." "" { target *-*-* } .-2 }

fn main() {}

