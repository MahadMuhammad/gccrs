#![crate_type = "lib"]
#![feature(register_tool)]
#![register_tool(xyz)]
#![warn(xyz::my_lint)] // this should not error
#![warn(abc::my_lint)]
// { dg-error ".E0710." "" { target *-*-* } .-1 }
// { help ".E0710." "" { target *-*-* } .-2 }
// { dg-error ".E0710." "" { target *-*-* } .-3 }
// { help ".E0710." "" { target *-*-* } .-4 }

