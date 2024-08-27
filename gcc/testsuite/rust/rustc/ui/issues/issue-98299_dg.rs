use std::convert::TryFrom;

pub fn test_usage(p: ()) {
    SmallCString::try_from(p).map(|cstr| cstr);
// { dg-error ".E0284." "" { target *-*-* } .-1 }
// { dg-error ".E0284." "" { target *-*-* } .-2 }
// { dg-error ".E0284." "" { target *-*-* } .-3 }
}

pub struct SmallCString<const N: usize> {}

impl<const N: usize> TryFrom<()> for SmallCString<N> {
    type Error = ();

    fn try_from(path: ()) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}

fn main() {}

