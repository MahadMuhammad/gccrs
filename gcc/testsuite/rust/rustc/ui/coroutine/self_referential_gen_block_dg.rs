//@ compile-flags: --edition 2024 -Zunstable-options
#![feature(gen_blocks)]
//! This test checks that we don't allow self-referential generators

fn main() {
    let mut x = {
        let mut x = gen {
            let y = 42;
            let z = &y; // { dg-error ".E0626." "" { target *-*-* } }
            yield 43;
            panic!("{z}");
        };
        x.next();
        Box::new(x)
    };
    x.next();
}

