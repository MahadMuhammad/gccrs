// { dg-additional-options "-frust-edition= 2021" }
//@ aux-build: empty.rs

// issue#128813

extern crate empty;

macro_rules! m {
    () => {
        extern crate std as empty;
// { dg-error ".E0259." "" { target *-*-* } .-1 }
    };
}

m!();

fn main() {
    use ::empty;
}

