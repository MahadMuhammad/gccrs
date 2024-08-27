trait Foo {}

trait Trait {
    type Associated;
}
trait DerivedTrait: Trait {}
trait GenericTrait<A> {
    type Associated;
}

struct Impl;
impl Foo for Impl {}
impl Trait for Impl {
    type Associated = ();
}
impl DerivedTrait for Impl {}
impl<A> GenericTrait<A> for Impl {
    type Associated = ();
}

fn returns_opaque() -> impl Trait + 'static {
    Impl
}
fn returns_opaque_derived() -> impl DerivedTrait + 'static {
    Impl
}
fn returns_opaque_foo() -> impl Trait + Foo {
    Impl
}
fn returns_opaque_derived_foo() -> impl DerivedTrait + Foo {
    Impl
}
fn returns_opaque_generic() -> impl GenericTrait<()> + 'static {
    Impl
}
fn returns_opaque_generic_foo() -> impl GenericTrait<()> + Foo {
    Impl
}
fn returns_opaque_generic_duplicate() -> impl GenericTrait<()> + GenericTrait<u8> {
    Impl
}

fn accepts_trait<T: Trait<Associated = ()>>(_: T) {}
fn accepts_generic_trait<T: GenericTrait<(), Associated = ()>>(_: T) {}

fn check_generics<A, B, C, D, E, F, G>(a: A, b: B, c: C, d: D, e: E, f: F, g: G)
where
    A: Trait + 'static,
    B: DerivedTrait + 'static,
    C: Trait + Foo,
    D: DerivedTrait + Foo,
    E: GenericTrait<()> + 'static,
    F: GenericTrait<()> + Foo,
    G: GenericTrait<()> + GenericTrait<u8>,
{
    accepts_trait(a);
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_trait(b);
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_trait(c);
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_trait(d);
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_generic_trait(e);
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_generic_trait(f);
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_generic_trait(g);
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

fn main() {
    accepts_trait(returns_opaque());
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_trait(returns_opaque_derived());
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_trait(returns_opaque_foo());
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_trait(returns_opaque_derived_foo());
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_generic_trait(returns_opaque_generic());
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_generic_trait(returns_opaque_generic_foo());
// { dg-error ".E0271." "" { target *-*-* } .-1 }

    accepts_generic_trait(returns_opaque_generic_duplicate());
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

