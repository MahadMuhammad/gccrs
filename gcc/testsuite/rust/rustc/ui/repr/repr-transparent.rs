// This file tests repr(transparent)-related errors reported during typeck. Other errors
// that are reported earlier and therefore preempt these are tested in:
// - repr-transparent-other-reprs.rs
// - repr-transparent-other-items.rs

#![feature(transparent_unions)]

use std::marker::PhantomData;

#[repr(transparent)]
struct NoFields;

#[repr(transparent)]
struct ContainsOnlyZst(());

#[repr(transparent)]
struct ContainsOnlyZstArray([bool; 0]);

#[repr(transparent)]
struct ContainsMultipleZst(PhantomData<*const i32>, NoFields);

#[repr(transparent)]
struct ContainsZstAndNonZst((), [i32; 2]);

#[repr(transparent)]
struct MultipleNonZst(u8, u8); // { dg-error ".E0690." "" { target *-*-* } }

trait Mirror { type It: ?Sized; }
impl<T: ?Sized> Mirror for T { type It = Self; }

#[repr(transparent)]
pub struct StructWithProjection(f32, <f32 as Mirror>::It);
// { dg-error ".E0690." "" { target *-*-* } .-1 }

#[repr(transparent)]
struct NontrivialAlignZst(u32, [u16; 0]); // { dg-error ".E0690." "" { target *-*-* } }

#[repr(align(32))]
struct ZstAlign32<T>(PhantomData<T>);

#[repr(transparent)]
struct GenericAlign<T>(ZstAlign32<T>, u32); // { dg-error ".E0690." "" { target *-*-* } }

#[repr(transparent)]
struct WrapsZstWithAlignment([i32; 0]);

#[repr(transparent)] // { dg-error ".E0084." "" { target *-*-* } }
enum Void {} // { dg-error ".E0731." "" { target *-*-* } }

#[repr(transparent)]
enum FieldlessEnum {
    Foo,
}

#[repr(transparent)]
enum UnitFieldEnum {
    Foo(()),
}

#[repr(transparent)]
enum TooManyFieldsEnum {
    Foo(u32, String),
}
// { dg-error ".E0690." "" { target *-*-* } .-3 }

#[repr(transparent)]
enum MultipleVariants { // { dg-error ".E0731." "" { target *-*-* } }
    Foo(String),
    Bar,
}

#[repr(transparent)]
enum NontrivialAlignZstEnum { // { dg-error ".E0690." "" { target *-*-* } }
    Foo(u32, [u16; 0]),
}

#[repr(transparent)]
enum GenericAlignEnum<T> { // { dg-error ".E0690." "" { target *-*-* } }
    Foo { bar: ZstAlign32<T>, baz: u32 }
}

#[repr(transparent)]
union UnitUnion {
    u: (),
}

#[repr(transparent)]
union TooManyFields { // { dg-error ".E0690." "" { target *-*-* } }
    u: u32,
    s: i32
}

fn main() {}

