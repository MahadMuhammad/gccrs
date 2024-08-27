//@ only-windows
//@ only-x86
#[link(name = "foo", import_name_type = "decorated")]
// { dg-error "" "" { target *-*-* } .-1 }
extern "C" { }

#[link(name = "bar", kind = "static", import_name_type = "decorated")]
// { dg-error "" "" { target *-*-* } .-1 }
extern "C" { }

// Specifying `import_name_type` before `kind` shouldn't raise an error.
#[link(name = "bar", import_name_type = "decorated", kind = "raw-dylib")]
extern "C" { }

fn main() {}

