//@ check-fail

struct Foo<'a> // { dg-error ".E0392." "" { target *-*-* } }
{
    _a: [u8; std::mem::size_of::<&'a mut u8>()] // { dg-error "" "" { target *-*-* } }
}

pub fn main() {}

