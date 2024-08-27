// { dg-additional-options "-frust-edition= 2021" }

// issue#128813

extern crate core as _;

macro_rules! m {
    () => {
        extern crate std as _;
    };
}

m!();

fn main() {
    use ::_;
// { dg-error "" "" { target *-*-* } .-1 }
}

