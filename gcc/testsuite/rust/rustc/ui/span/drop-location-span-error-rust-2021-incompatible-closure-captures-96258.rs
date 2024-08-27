#![warn(rust_2021_incompatible_closure_captures)]

fn main() {}

pub(crate) struct Numberer {}

impl Numberer {
    pub(crate) async fn new(
// { dg-error ".E0670." "" { target *-*-* } .-1 }
        interval: Duration,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    ) -> Numberer {
        Numberer {}
    }
}

