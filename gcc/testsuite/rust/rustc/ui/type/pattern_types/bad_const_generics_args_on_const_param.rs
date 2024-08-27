#![feature(pattern_types, core_pattern_type)]
#![allow(internal_features)]

type Pat<const START: u32, const END: u32> =
    std::pat::pattern_type!(u32 is START::<(), i32, 2>..=END::<_, Assoc = ()>);
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { dg-error ".E0229." "" { target *-*-* } .-2 }
// { dg-error ".E0229." "" { target *-*-* } .-3 }

fn main() {}

