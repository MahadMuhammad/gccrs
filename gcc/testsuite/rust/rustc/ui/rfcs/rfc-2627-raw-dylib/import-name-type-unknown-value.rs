//@ only-windows
//@ only-x86
#[link(name = "foo", kind = "raw-dylib", import_name_type = "unknown")]
// { dg-error "" "" { target *-*-* } .-1 }
extern "C" { }

fn main() {}

