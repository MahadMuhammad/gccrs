// Make sure we honor region constraints when normalizing type annotations.

//@ check-fail

#![feature(more_qualified_paths)]

trait Trait {
    type Assoc;
}

impl<T> Trait for T
where
    T: 'static,
{
    type Assoc = MyTy<()>;
}

enum MyTy<T> {
    Unit,
    Tuple(),
    Struct {},
    Dumb(T),
}

impl<T> MyTy<T> {
    const CONST: () = ();
    fn method<X>() {}
    fn method2<X>(&self) {}
}

trait TraitAssoc {
    const TRAIT_CONST: ();
    fn trait_method<X>(&self);
}
impl<T> TraitAssoc for T {
    const TRAIT_CONST: () = ();
    fn trait_method<X>(&self) {}
}

type Ty<'a> = <&'a () as Trait>::Assoc;

fn test_local<'a>() {
    let _: Ty<'a> = MyTy::Unit;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test_closure_sig<'a, 'b>() {
    |_: Ty<'a>| {};
// { dg-error "" "" { target *-*-* } .-1 }
    || -> Option<Ty<'b>> { None };
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test_path<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>() {
    <Ty<'a>>::method::<Ty<'static>>;
// { dg-error "" "" { target *-*-* } .-1 }
    <Ty<'static>>::method::<Ty<'b>>;
// { dg-error "" "" { target *-*-* } .-1 }

    <Ty<'c>>::trait_method::<Ty<'static>>;
// { dg-error "" "" { target *-*-* } .-1 }
    <Ty<'static>>::trait_method::<Ty<'d>>;
// { dg-error "" "" { target *-*-* } .-1 }

    <Ty<'e>>::CONST;
// { dg-error "" "" { target *-*-* } .-1 }
    <Ty<'f>>::TRAIT_CONST;
// { dg-error "" "" { target *-*-* } .-1 }

    <Ty<'static>>::method::<Ty<'static>>;
    <Ty<'static>>::trait_method::<Ty<'static>>;
    <Ty<'static>>::CONST;
    <Ty<'static>>::TRAIT_CONST;

    MyTy::Unit::<Ty<'g>>;
// { dg-error "" "" { target *-*-* } .-1 }
    MyTy::<Ty<'h>>::Unit;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test_call<'a, 'b, 'c>() {
    <Ty<'a>>::method::<Ty<'static>>();
// { dg-error "" "" { target *-*-* } .-1 }
    <Ty<'static>>::method::<Ty<'b>>();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test_variants<'a, 'b, 'c>() {
    <Ty<'a>>::Struct {};
// { dg-error "" "" { target *-*-* } .-1 }
    <Ty<'b>>::Tuple();
// { dg-error "" "" { target *-*-* } .-1 }
    <Ty<'c>>::Unit;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test_method_call<'a, 'b>(x: MyTy<()>) {
    x.method2::<Ty<'a>>();
// { dg-error "" "" { target *-*-* } .-1 }
    x.trait_method::<Ty<'b>>();
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test_struct_path<'a, 'b, 'c, 'd>() {
    struct Struct<T> { x: Option<T>, }

    trait Project {
        type Struct;
        type Enum;
    }
    impl<T> Project for T {
        type Struct = Struct<()>;
        type Enum = MyTy<()>;
    }

    // Resolves to enum variant
    MyTy::<Ty<'a>>::Struct {}; // without SelfTy
// { dg-error "" "" { target *-*-* } .-1 }
    <Ty<'b> as Project>::Enum::Struct {}; // with SelfTy
// { dg-error "" "" { target *-*-* } .-1 }

    // Resolves to struct and associated type respectively
    Struct::<Ty<'c>> { x: None, }; // without SelfTy
// { dg-error "" "" { target *-*-* } .-1 }
    <Ty<'d> as Project>::Struct { x: None, }; // with SelfTy
// { dg-error "" "" { target *-*-* } .-1 }
}

fn test_pattern<'a, 'b, 'c, 'd, 'e, 'f>() {
    use MyTy::*;
    match MyTy::Unit {
        Struct::<Ty<'a>> {..} => {},
// { dg-error "" "" { target *-*-* } .-1 }
        Tuple::<Ty<'b>> (..) => {},
// { dg-error "" "" { target *-*-* } .-1 }
        Unit::<Ty<'c>> => {},
// { dg-error "" "" { target *-*-* } .-1 }
        Dumb(_) => {},
    };
    match MyTy::Unit {
        <Ty<'d>>::Struct {..} => {},
// { dg-error "" "" { target *-*-* } .-1 }
        <Ty<'e>>::Tuple (..) => {},
// { dg-error "" "" { target *-*-* } .-1 }
        <Ty<'f>>::Unit => {},
// { dg-error "" "" { target *-*-* } .-1 }
        Dumb(_) => {},
    };
}


fn main() {}

