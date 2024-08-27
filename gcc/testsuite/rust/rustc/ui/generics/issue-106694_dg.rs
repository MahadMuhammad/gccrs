trait Trait {}

fn foo(_: impl &Trait) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn bar<T: &Trait>(_: T) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn partially_correct_impl(_: impl &*const &Trait + Copy) {}
// { dg-error "" "" { target *-*-* } .-1 }

fn foo_bad(_: impl &BadTrait) {}
// { dg-error ".E0405." "" { target *-*-* } .-1 }
// { dg-error ".E0405." "" { target *-*-* } .-2 }

fn bar_bad<T: &BadTrait>(_: T) {}
// { dg-error ".E0405." "" { target *-*-* } .-1 }
// { dg-error ".E0405." "" { target *-*-* } .-2 }

fn partially_correct_impl_bad(_: impl &*const &BadTrait + Copy) {}
// { dg-error ".E0405." "" { target *-*-* } .-1 }
// { dg-error ".E0405." "" { target *-*-* } .-2 }

fn main() {}

