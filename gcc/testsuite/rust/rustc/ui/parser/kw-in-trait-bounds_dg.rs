// { dg-additional-options "-frust-edition=2018" }

fn _f<F: fn(), G>(_: impl fn(), _: &dyn fn())
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { help "" "" { target *-*-* } .-4 }
// { help "" "" { target *-*-* } .-5 }
// { help "" "" { target *-*-* } .-6 }
where
G: fn(),
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
{}

fn _g<A: struct, B>(_: impl struct, _: &dyn struct)
// { dg-error ".E0405." "" { target *-*-* } .-1 }
// { dg-error ".E0405." "" { target *-*-* } .-2 }
// { dg-error ".E0405." "" { target *-*-* } .-3 }
// { dg-error ".E0405." "" { target *-*-* } .-4 }
// { dg-error ".E0405." "" { target *-*-* } .-5 }
// { dg-error ".E0405." "" { target *-*-* } .-6 }
// { help ".E0405." "" { target *-*-* } .-7 }
// { help ".E0405." "" { target *-*-* } .-8 }
// { help ".E0405." "" { target *-*-* } .-9 }
// { help ".E0405." "" { target *-*-* } .-10 }
// { help ".E0405." "" { target *-*-* } .-11 }
// { help ".E0405." "" { target *-*-* } .-12 }
where
    B: struct,
// { dg-error ".E0405." "" { target *-*-* } .-1 }
// { dg-error ".E0405." "" { target *-*-* } .-2 }
// { help ".E0405." "" { target *-*-* } .-3 }
// { help ".E0405." "" { target *-*-* } .-4 }
{}

trait Struct {}

fn main() {}

