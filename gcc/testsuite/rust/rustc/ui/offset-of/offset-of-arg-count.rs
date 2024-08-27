use std::mem::offset_of;

fn main() {
    offset_of!(NotEnoughArguments); // { dg-error "" "" { target *-*-* } }
    offset_of!(NotEnoughArgumentsWithAComma, ); // { dg-error "" "" { target *-*-* } }
    offset_of!(Container, field, too many arguments); // { dg-error "" "" { target *-*-* } }
    offset_of!(S, f); // compiles fine
    offset_of!(S, f,); // also compiles fine
    offset_of!(S, f.); // { dg-error "" "" { target *-*-* } }
    offset_of!(S, f.,); // { dg-error "" "" { target *-*-* } }
    offset_of!(S, f..); // { dg-error "" "" { target *-*-* } }
    offset_of!(S, f..,); // { dg-error "" "" { target *-*-* } }
    offset_of!(Lt<'static>, bar); // issue #111657
    offset_of!(Lt<'_>, bar); // issue #111678
}

struct S { f: u8, }
struct Lt<'a> {
    bar: &'a (),
}

