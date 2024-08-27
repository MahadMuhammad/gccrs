// Weak alias types only constrain late-bound regions if their normalized form constrains them.

#![feature(lazy_type_alias)]
#![allow(incomplete_features)]

type NotInjective<'a> = <() as Discard>::Output<'a>;

type FnPtr0 = for<'a> fn(NotInjective<'a>) -> &'a ();
// { dg-error ".E0581." "" { target *-*-* } .-1 }
type FnPtr1 = for<'a> fn(NotInjectiveEither<'a, ()>) -> NotInjectiveEither<'a, ()>;
// { dg-error ".E0581." "" { target *-*-* } .-1 }
type DynCl = dyn for<'a> Fn(NotInjective<'a>) -> &'a ();
// { dg-error ".E0582." "" { target *-*-* } .-1 }

trait Discard { type Output<'a>; }
impl Discard for () { type Output<'_a> = (); }

type NotInjectiveEither<'a, Linchpin> = Linchpin
where
    Linchpin: Fn() -> &'a ();


fn main() {}

