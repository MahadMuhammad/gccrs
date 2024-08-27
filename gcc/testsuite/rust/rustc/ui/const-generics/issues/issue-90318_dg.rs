#![feature(const_type_id)]
#![feature(generic_const_exprs)]
#![feature(core_intrinsics)]
#![allow(incomplete_features)]

use std::any::TypeId;

struct If<const B: bool>;
pub trait True {}
impl True for If<true> {}

fn consume<T: 'static>(_val: T)
where
    If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
{
}

fn test<T: 'static>()
where
    If<{ TypeId::of::<T>() != TypeId::of::<()>() }>: True,
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
{
}

fn main() {
    let a = ();
    consume(0i32);
    consume(a);
}

