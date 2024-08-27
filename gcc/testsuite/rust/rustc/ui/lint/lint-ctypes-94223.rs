#![crate_type = "lib"]
#![deny(improper_ctypes_definitions)]

pub fn bad(f: extern "C" fn([u8])) {}
// { dg-error "" "" { target *-*-* } .-1 }

pub fn bad_twice(f: Result<extern "C" fn([u8]), extern "C" fn([u8])>) {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

struct BadStruct(extern "C" fn([u8]));
// { dg-error "" "" { target *-*-* } .-1 }

enum BadEnum {
    A(extern "C" fn([u8])),
// { dg-error "" "" { target *-*-* } .-1 }
}

enum BadUnion {
    A(extern "C" fn([u8])),
// { dg-error "" "" { target *-*-* } .-1 }
}

type Foo = extern "C" fn([u8]);
// { dg-error "" "" { target *-*-* } .-1 }

pub trait FooTrait {
    type FooType;
}

pub type Foo2<T> = extern "C" fn(Option<&<T as FooTrait>::FooType>);
// { dg-error "" "" { target *-*-* } .-1 }

pub struct FfiUnsafe;

#[allow(improper_ctypes_definitions)]
extern "C" fn f(_: FfiUnsafe) {
    unimplemented!()
}

pub static BAD: extern "C" fn(FfiUnsafe) = f;
// { dg-error "" "" { target *-*-* } .-1 }

pub static BAD_TWICE: Result<extern "C" fn(FfiUnsafe), extern "C" fn(FfiUnsafe)> = Ok(f);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

pub const BAD_CONST: extern "C" fn(FfiUnsafe) = f;
// { dg-error "" "" { target *-*-* } .-1 }

