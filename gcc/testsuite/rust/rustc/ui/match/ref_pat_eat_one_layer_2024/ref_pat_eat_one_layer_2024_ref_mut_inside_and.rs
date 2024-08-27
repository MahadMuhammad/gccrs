// { dg-additional-options "-frust-edition= 2024" }
//@ compile-flags: -Zunstable-options
//@ run-rustfix
#![allow(incomplete_features)]
#![feature(ref_pat_eat_one_layer_2024)]

pub fn main() {
    if let Some(&Some(ref mut x)) = &mut Some(Some(0)) {
// { dg-error ".E0596." "" { target *-*-* } .-1 }
        let _: &mut u8 = x;
    }

    if let &Some(Some(ref mut x)) = &mut Some(Some(0)) {
// { dg-error ".E0596." "" { target *-*-* } .-1 }
        let _: &mut u8 = x;
    }

    macro_rules! pat {
        ($var:ident) => { ref mut $var };
    }
    let &pat!(x) = &mut 0;
// { dg-error ".E0596." "" { target *-*-* } .-1 }
    let _: &mut u8 = x;

    let &(ref mut a, ref mut b) = &mut (true, false);
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { dg-error ".E0596." "" { target *-*-* } .-2 }
    let _: &mut bool = a;
    let _: &mut bool = b;
}

