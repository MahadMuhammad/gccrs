//@ run-rustfix
// { dg-additional-options "-frust-edition=2018" }
//@ check-pass
#![warn(rust_2021_prelude_collisions)]

trait TryIntoU32 {
    fn try_into(self) -> Result<u32, ()>;
}

impl TryIntoU32 for u8 {
    fn try_into(self) -> Result<u32, ()> {
        Ok(self as u32)
    }
}

// needed for autoref test
impl TryIntoU32 for &f32 {
    fn try_into(self) -> Result<u32, ()> {
        Ok(*self as u32)
    }
}

trait TryFromU8: Sized {
    fn try_from(x: u8) -> Result<Self, ()>;
}

impl TryFromU8 for u32 {
    fn try_from(x: u8) -> Result<Self, ()> {
        Ok(x as u32)
    }
}

impl TryIntoU32 for *const u16 {
    fn try_into(self) -> Result<u32, ()> {
        Ok(unsafe { *self } as u32)
    }
}

trait FromByteIterator {
    fn from_iter<T>(iter: T) -> Self
    where
        T: Iterator<Item = u8>;
}

impl FromByteIterator for Vec<u8> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: Iterator<Item = u8>,
    {
        iter.collect()
    }
}

fn main() {
    // test dot-call that will break in 2021 edition
    let _: u32 = 3u8.try_into().unwrap();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    // test associated function call that will break in 2021 edition
    let _ = u32::try_from(3u8).unwrap();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    // test reverse turbofish too
    let _ = <Vec<u8>>::from_iter(vec![1u8, 2, 3, 4, 5, 6].into_iter());
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    // negative testing lint (this line should *not* emit a warning)
    let _: u32 = TryFromU8::try_from(3u8).unwrap();

    // test type omission
    let _: u32 = <_>::try_from(3u8).unwrap();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    // test autoderef
    let _: u32 = (&3u8).try_into().unwrap();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    // test autoref
    let _: u32 = 3.0.try_into().unwrap();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    let mut data = 3u16;
    let mut_ptr = std::ptr::addr_of_mut!(data);
    let _: u32 = mut_ptr.try_into().unwrap();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }

    type U32Alias = u32;
    let _ = U32Alias::try_from(3u8).unwrap();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

