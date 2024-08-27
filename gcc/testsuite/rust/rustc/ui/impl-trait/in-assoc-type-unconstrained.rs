#![feature(impl_trait_in_assoc_type)]

mod compare_ty {
    trait Trait {
        type Ty: IntoIterator<Item = ()>;
    }
    impl Trait for () {
        type Ty = Option<impl Sized>;
// { dg-error ".E0271." "" { target *-*-* } .-1 }
// { dg-error ".E0271." "" { target *-*-* } .-2 }
    }
}

mod compare_method {
    trait Trait {
        type Ty;
        fn method() -> Self::Ty;
    }
    impl Trait for () {
        type Ty = impl Sized;
// { dg-error "" "" { target *-*-* } .-1 }
        fn method() -> () {}
// { dg-error ".E0053." "" { target *-*-* } .-1 }
    }
}

fn main() {}

