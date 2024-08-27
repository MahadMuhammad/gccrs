#![crate_type = "rlib"]
#![feature(generic_arg_infer)]

struct Foo<const N: usize>;
struct Bar<T, const N: usize>(T);

fn arr_fn() -> [u8; _] {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
    [0; 3]
}

fn ty_fn() -> Bar<i32, _> {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
    Bar::<i32, 3>(0)
}

fn ty_fn_mixed() -> Bar<_, _> {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
    Bar::<i32, 3>(0)
}

const ARR_CT: [u8; _] = [0; 3];
// { dg-error ".E0121." "" { target *-*-* } .-1 }
static ARR_STATIC: [u8; _] = [0; 3];
// { dg-error ".E0121." "" { target *-*-* } .-1 }
const TY_CT: Bar<i32, _> = Bar::<i32, 3>(0);
// { dg-error ".E0121." "" { target *-*-* } .-1 }
static TY_STATIC: Bar<i32, _> = Bar::<i32, 3>(0);
// { dg-error ".E0121." "" { target *-*-* } .-1 }
const TY_CT_MIXED: Bar<_, _> = Bar::<i32, 3>(0);
// { dg-error ".E0121." "" { target *-*-* } .-1 }
static TY_STATIC_MIXED: Bar<_, _> = Bar::<i32, 3>(0);
// { dg-error ".E0121." "" { target *-*-* } .-1 }
trait ArrAssocConst {
    const ARR: [u8; _];
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}
trait TyAssocConst {
    const ARR: Bar<i32, _>;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}
trait TyAssocConstMixed {
    const ARR: Bar<_, _>;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}

trait AssocTy {
    type Assoc;
}
impl AssocTy for i8 {
    type Assoc = [u8; _];
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}
impl AssocTy for i16 {
    type Assoc = Bar<i32, _>;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}
impl AssocTy for i32 {
    type Assoc = Bar<_, _>;
// { dg-error ".E0121." "" { target *-*-* } .-1 }
}

