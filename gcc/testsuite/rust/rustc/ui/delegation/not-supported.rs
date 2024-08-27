#![feature(const_trait_impl)]
#![feature(c_variadic)]
#![feature(effects)]
#![feature(fn_delegation)]
#![allow(incomplete_features)]

mod generics {
    trait GenericTrait<T> {
        fn bar(&self, x: T) -> T { x }
        fn bar1() {}
    }
    trait Trait {
        fn foo(&self, x: i32) -> i32 { x }
        fn foo1<'a>(&self, x: &'a i32) -> &'a i32 { x }
        fn foo2<T>(&self, x: T) -> T { x }
        fn foo3<'a: 'a>(_: &'a u32) {}

        reuse GenericTrait::bar;
// { dg-error "" "" { target *-*-* } .-1 }
        reuse GenericTrait::bar1;
// { dg-error "" "" { target *-*-* } .-1 }
    }

    struct F;
    impl Trait for F {}
    impl<T> GenericTrait<T> for F {}

    struct S(F);

    impl<T> GenericTrait<T> for S {
        reuse <F as GenericTrait<T>>::bar { &self.0 }
// { dg-error "" "" { target *-*-* } .-1 }
        reuse GenericTrait::<T>::bar1;
// { dg-error "" "" { target *-*-* } .-1 }
    }

    impl GenericTrait<()> for () {
        reuse GenericTrait::bar { &F }
// { dg-error "" "" { target *-*-* } .-1 }
        reuse GenericTrait::bar1;
// { dg-error "" "" { target *-*-* } .-1 }
    }

    impl Trait for &S {
        reuse Trait::foo;
// { dg-error "" "" { target *-*-* } .-1 }
    }

    impl Trait for S {
        reuse Trait::foo1 { &self.0 }
        reuse Trait::foo2 { &self.0 }
// { dg-error ".E0049." "" { target *-*-* } .-1 }
// { dg-error ".E0049." "" { target *-*-* } .-2 }
        reuse <F as Trait>::foo3;
// { dg-error ".E0195." "" { target *-*-* } .-1 }
// { dg-error ".E0195." "" { target *-*-* } .-2 }
    }

    struct GenericS<T>(T);
    impl<T> Trait for GenericS<T> {
        reuse Trait::foo { &self.0 }
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

mod opaque {
    trait Trait {}
    impl Trait for () {}

    mod to_reuse {
        use super::Trait;

        pub fn opaque_ret() -> impl Trait { unimplemented!() }
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    }

    trait ToReuse {
        fn opaque_ret() -> impl Trait { unimplemented!() }
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    }

    // FIXME: Inherited `impl Trait`s create query cycles when used inside trait impls.
    impl ToReuse for u8 {
        reuse to_reuse::opaque_ret; // { dg-error ".E0391." "" { target *-*-* } }
    }
    impl ToReuse for u16 {
        reuse ToReuse::opaque_ret; // { dg-error ".E0391." "" { target *-*-* } }
    }
}

mod recursive {
    mod to_reuse1 {
        pub mod to_reuse2 {
            pub fn foo() {}
        }

        pub reuse to_reuse2::foo;
    }

    reuse to_reuse1::foo;
// { dg-error "" "" { target *-*-* } .-1 }
}

mod effects {
    #[const_trait]
    trait Trait {
        fn foo();
    }

    reuse Trait::foo;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

