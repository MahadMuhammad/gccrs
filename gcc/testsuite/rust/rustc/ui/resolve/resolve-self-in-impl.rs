#![feature(associated_type_defaults)]

struct S<T = u8>(T);
trait Tr<T = u8> {
    type A = ();
}

impl Tr<Self> for S {} // OK
impl<T: Tr<Self>> Tr<T> for S {} // OK
impl Tr for S where Self: Copy {} // OK
impl Tr for S where S<Self>: Copy {} // OK
impl Tr for S where Self::A: Copy {} // OK

impl Tr for Self {} // { dg-error "" "" { target *-*-* } }
impl Tr for S<Self> {} // { dg-error "" "" { target *-*-* } }
impl Self {} // { dg-error "" "" { target *-*-* } }
impl S<Self> {} // { dg-error "" "" { target *-*-* } }
impl (Self, Self) {} // { dg-error "" "" { target *-*-* } }
impl Tr<Self::A> for S {} // { dg-error ".E0391." "" { target *-*-* } }

fn main() {}

