// Make sure that trying to access `TryInto`, `TryFrom`, `FromIterator` in pre-2021 mentions
// Edition 2021 change.
// { dg-additional-options "-frust-edition=2018" }

fn test() {
    let _i: i16 = 0_i32.try_into().unwrap();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-note ".E0599." "" { target *-*-* } .-2 }

    let _i: i16 = TryFrom::try_from(0_i32).unwrap();
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-note ".E0433." "" { target *-*-* } .-2 }
// { dg-note ".E0433." "" { target *-*-* } .-3 }

    let _i: i16 = TryInto::try_into(0_i32).unwrap();
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-note ".E0433." "" { target *-*-* } .-2 }
// { dg-note ".E0433." "" { target *-*-* } .-3 }

    let _v: Vec<_> = FromIterator::from_iter(&[1]);
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-note ".E0433." "" { target *-*-* } .-2 }
// { dg-note ".E0433." "" { target *-*-* } .-3 }
}

fn main() {
    test();
}

