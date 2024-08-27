// { dg-additional-options "-frust-edition= 2024" }
//@ compile-flags: -Zunstable-options
#![allow(incomplete_features)]
#![feature(ref_pat_eat_one_layer_2024)]

pub fn main() {
    if let Some(&Some(x)) = Some(&Some(&mut 0)) {
// { dg-error ".E0507." "" { target *-*-* } .-1 }
        let _: &u32 = x;
    }

    let &ref mut x = &0;
// { dg-error ".E0596." "" { target *-*-* } .-1 }
}

