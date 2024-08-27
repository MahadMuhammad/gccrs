#![feature(builtin_syntax)]

use std::mem::offset_of;

fn main() {
    offset_of!((u8, u8), _0); // { dg-error ".E0609." "" { target *-*-* } }
    offset_of!((u8, u8), 01); // { dg-error ".E0609." "" { target *-*-* } }
    offset_of!((u8, u8), 1e2); // { dg-error ".E0609." "" { target *-*-* } }
    offset_of!((u8, u8), 1_u8); // { dg-error ".E0609." "" { target *-*-* } }
// { dg-error ".E0609." "" { target *-*-* } .-2 }
    offset_of!((u8, u8), +1); // { dg-error "" "" { target *-*-* } }
    offset_of!((u8, u8), -1); // { dg-error "" "" { target *-*-* } }
    offset_of!((u8, u8), 1.); // { dg-error "" "" { target *-*-* } }
    offset_of!((u8, u8), 1 .); // { dg-error "" "" { target *-*-* } }
    builtin # offset_of((u8, u8), 1e2); // { dg-error ".E0609." "" { target *-*-* } }
    builtin # offset_of((u8, u8), _0); // { dg-error ".E0609." "" { target *-*-* } }
    builtin # offset_of((u8, u8), 01); // { dg-error ".E0609." "" { target *-*-* } }
    builtin # offset_of((u8, u8), 1_u8); // { dg-error ".E0609." "" { target *-*-* } }
// { dg-error ".E0609." "" { target *-*-* } .-2 }
    // We need to put these into curly braces, otherwise only one of the
    // errors will be emitted and the others suppressed.
    { builtin # offset_of((u8, u8), +1) }; // { dg-error "" "" { target *-*-* } }
    { builtin # offset_of((u8, u8), 1.) }; // { dg-error "" "" { target *-*-* } }
    { builtin # offset_of((u8, u8), 1 .) }; // { dg-error "" "" { target *-*-* } }
}

type ComplexTup = (((u8, u8), u8), u8);

fn nested() {
    offset_of!(((u8, u16), (u32, u16, u8)), 0.2); // { dg-error ".E0609." "" { target *-*-* } }
    offset_of!(((u8, u16), (u32, u16, u8)), 1.2);
    offset_of!(((u8, u16), (u32, u16, u8)), 1.2.0); // { dg-error ".E0609." "" { target *-*-* } }

    // All combinations of spaces (this sends different tokens to the parser)
    offset_of!(ComplexTup, 0.0.1.); // { dg-error "" "" { target *-*-* } }
    offset_of!(ComplexTup, 0 .0.1.); // { dg-error "" "" { target *-*-* } }
    offset_of!(ComplexTup, 0 . 0.1.); // { dg-error "" "" { target *-*-* } }
    offset_of!(ComplexTup, 0. 0.1.); // { dg-error "" "" { target *-*-* } }
    offset_of!(ComplexTup, 0.0 .1.); // { dg-error "" "" { target *-*-* } }
    offset_of!(ComplexTup, 0.0 . 1.); // { dg-error "" "" { target *-*-* } }
    offset_of!(ComplexTup, 0.0. 1.); // { dg-error "" "" { target *-*-* } }

    // Test for builtin too to ensure that the builtin syntax can also handle these cases
    // We need to put these into curly braces, otherwise only one of the
    // errors will be emitted and the others suppressed.
    { builtin # offset_of(ComplexTup, 0.0.1.) }; // { dg-error "" "" { target *-*-* } }
    { builtin # offset_of(ComplexTup, 0 .0.1.) }; // { dg-error "" "" { target *-*-* } }
    { builtin # offset_of(ComplexTup, 0 . 0.1.) }; // { dg-error "" "" { target *-*-* } }
    { builtin # offset_of(ComplexTup, 0. 0.1.) }; // { dg-error "" "" { target *-*-* } }
    { builtin # offset_of(ComplexTup, 0.0 .1.) }; // { dg-error "" "" { target *-*-* } }
    { builtin # offset_of(ComplexTup, 0.0 . 1.) }; // { dg-error "" "" { target *-*-* } }
    { builtin # offset_of(ComplexTup, 0.0. 1.) }; // { dg-error "" "" { target *-*-* } }
}

