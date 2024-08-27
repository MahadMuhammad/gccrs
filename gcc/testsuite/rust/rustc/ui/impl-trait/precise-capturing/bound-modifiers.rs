// { dg-additional-options "-frust-edition= 2021" }

fn polarity() -> impl Sized + ?use<> {}
// { dg-error ".E0405." "" { target *-*-* } .-1 }
// { dg-error ".E0405." "" { target *-*-* } .-2 }
// { dg-warning ".E0405." "" { target *-*-* } .-3 }
// { dg-warning ".E0405." "" { target *-*-* } .-4 }

fn asyncness() -> impl Sized + async use<> {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }

fn constness() -> impl Sized + const use<> {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }

fn binder() -> impl Sized + for<'a> use<> {}
// { dg-error ".E0405." "" { target *-*-* } .-1 }
// { dg-error ".E0405." "" { target *-*-* } .-2 }

fn main() {}

