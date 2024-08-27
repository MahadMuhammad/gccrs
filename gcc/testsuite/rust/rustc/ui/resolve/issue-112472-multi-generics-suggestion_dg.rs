//@ run-rustfix

use std::fmt::Debug;
use std::marker::PhantomData;
use std::convert::{self, TryFrom};

#[allow(unused)]
struct Codec<EncodeLine, DecodeLine> {
    phantom_decode: PhantomData<DecodeLine>,
    phantom_encode: PhantomData<EncodeLine>,
}

pub enum ParseError {}

impl<EncodeLine, DecodeLine> Codec<EncodeLine, DecodeLine> where
    DecodeLine: Debug + convert::TryFrom<String>,
    <DecodeLine as convert::TryFrom<String>>::Error: ParseError,
// { dg-error ".E0404." "" { target *-*-* } .-1 }
// { help ".E0404." "" { target *-*-* } .-2 }
{
}

impl<EncodeLine, DecodeLine> Codec<EncodeLine, DecodeLine> where
    DecodeLine: Debug + TryFrom<String>,
    <DecodeLine as TryFrom<String>>::Error: ParseError,
// { dg-error ".E0404." "" { target *-*-* } .-1 }
// { help ".E0404." "" { target *-*-* } .-2 }
{
}

fn main() {}

