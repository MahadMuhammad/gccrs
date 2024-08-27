// Ensure that the future_incompatible lint group only includes
// lints for changes that are not tied to an edition
#![deny(future_incompatible)]

trait Tr {
    // Warn only since this is not a `future_incompatible` lint
    fn f(u8) {} // { dg-warning "" "" { target *-*-* } }
// { dg-warning "" "" { target *-*-* } .-2 }
}

pub mod submodule {
    // Error since this is a `future_incompatible` lint
    #![doc(test(some_test))]
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

