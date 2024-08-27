// { dg-additional-options "-frust-edition= 2021" }

// issue#128813

extern crate non_existent;
// { dg-error ".E0463." "" { target *-*-* } .-1 }

macro_rules! m {
    () => {
        extern crate std as non_existent;
// { dg-error ".E0259." "" { target *-*-* } .-1 }
    };
}

m!();

fn main() {
    use ::non_existent;
}

