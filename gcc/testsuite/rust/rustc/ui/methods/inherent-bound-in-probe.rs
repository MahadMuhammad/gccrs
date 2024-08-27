//@ normalize-stderr-test: "long-type-\d+" -> "long-type-hash"

// Fixes #110131
//
// The issue is that we were constructing an `ImplDerived` cause code for the
// `&'a T: IntoIterator<Item = &'a u8>` obligation for `Helper::new`, which is
// incorrect because derived obligations are only expected to come from *traits*.

struct SeqBuffer<'a, T>
where
    &'a T: IntoIterator<Item = &'a u8>,
{
    iter: <&'a T as IntoIterator>::IntoIter,
}

struct Helper<'a, T>
where
    &'a T: IntoIterator<Item = &'a u8>,
{
    buf: SeqBuffer<'a, T>,
}

impl<'a, T> Helper<'a, T>
where
    &'a T: IntoIterator<Item = &'a u8>,
{
    fn new(sq: &'a T) -> Self {
        loop {}
    }
}

struct BitReaderWrapper<T>(T);

impl<'a, T> IntoIterator for &'a BitReaderWrapper<T>
where
    &'a T: IntoIterator<Item = &'a u8>,
{
    type Item = u32;

    type IntoIter = Helper<'a, T>;
// { dg-error ".E0277." "" { target *-*-* } .-1 }

    fn into_iter(self) -> Self::IntoIter {
        Helper::new(&self.0)
// { dg-error ".E0275." "" { target *-*-* } .-1 }
    }
}

fn main() {}

