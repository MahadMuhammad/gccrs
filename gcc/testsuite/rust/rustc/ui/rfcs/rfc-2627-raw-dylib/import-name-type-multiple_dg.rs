// ignore-tidy-linelength
//@ only-windows
//@ only-x86
#[link(name = "foo", kind = "raw-dylib", import_name_type = "decorated", import_name_type = "decorated")]
// { dg-error "" "" { target *-*-* } .-1 }
extern "C" { }

fn main() {}

