// issue #100631, make sure `TyCtxt::get_attr` only called by case that compiler
// can reasonably deal with multiple attributes.
// `repr` will use `TyCtxt::get_attrs` since it's `DuplicatesOk`.
#[repr(C)] // { dg-error ".E0084." "" { target *-*-* } }
#[repr(C)]
enum Foo {}

fn main() {}

