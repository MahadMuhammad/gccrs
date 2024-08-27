//@ run-pass
// { dg-additional-options "-frust-edition= 2024" }
//@ compile-flags: -Zunstable-options

#![allow(incomplete_features)]
#![feature(ref_pat_eat_one_layer_2024)]

struct Foo;
// { dg-warning "" "" { target *-*-* } .-1 }

fn main() {
    || {
// { dg-warning "" "" { target *-*-* } .-1 }
        if let Some(Some(&mut x)) = &mut Some(&mut Some(0)) {
            let _: u32 = x;
        }
    };
}

