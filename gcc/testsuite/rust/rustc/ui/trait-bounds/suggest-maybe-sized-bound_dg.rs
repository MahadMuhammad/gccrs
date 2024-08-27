// issue: 120878
fn main() {
    struct StructA<A, B = A> {
        _marker: std::marker::PhantomData<fn() -> (A, B)>,
    }

    struct StructB {
        a: StructA<isize, [u8]>,
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    }

    trait Trait {
        type P<X>;
    }

    impl Trait for () {
        type P<X> = [u8];
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    }
}

