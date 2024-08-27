// { dg-additional-options "-frust-edition= 2021" }

// issue#128813

extern crate core;

macro_rules! m {
    () => {
        extern crate std as core;
// { dg-error ".E0259." "" { target *-*-* } .-1 }
    };
}

m!();

fn main() {
    use ::core;
}

