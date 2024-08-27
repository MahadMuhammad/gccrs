#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

struct Family<T>(T);

impl Family<()> {
    type Proj = ();
}

impl<T> Family<Result<T, ()>> {
    type Proj = Self;
}

fn main() {
    let _: Family<Option<()>>::Proj; // { dg-error ".E0220." "" { target *-*-* } }
    let _: Family<std::path::PathBuf>::Proj = (); // { dg-error ".E0220." "" { target *-*-* } }
}

