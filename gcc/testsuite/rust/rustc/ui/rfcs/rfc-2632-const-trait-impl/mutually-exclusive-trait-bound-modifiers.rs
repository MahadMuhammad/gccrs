#![feature(const_trait_impl)]

const fn maybe_const_maybe<T: ~const ?Sized>() {}
// { dg-error "" "" { target *-*-* } .-1 }

fn const_maybe<T: const ?Sized>() {}
// { dg-error "" "" { target *-*-* } .-1 }

const fn maybe_const_negative<T: ~const !Trait>() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

fn const_negative<T: const !Trait>() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }

#[const_trait]
trait Trait {}

fn main() {}

