//@ run-rustfix

struct Foo;

impl std::ops::Mul for &Foo {
    type Output = Foo;

    fn mul(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

fn main() {
    let ref_mut_foo: &mut Foo = &mut Foo;
    let ref_foo: &Foo = &Foo;
    let owned_foo: Foo = Foo;

    let _ = ref_foo * ref_foo;
    let _ = ref_foo * ref_mut_foo;

    let _ = ref_mut_foo * ref_foo;
// { dg-error ".E0369." "" { target *-*-* } .-1 }
    let _ = ref_mut_foo * ref_mut_foo;
// { dg-error ".E0369." "" { target *-*-* } .-1 }
    let _ = ref_mut_foo * &owned_foo;
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

