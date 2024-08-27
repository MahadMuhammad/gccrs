// Regression test for #109072.
// Check that we don't ICE when canonicalizing user annotation.

trait Lt<'a> {
    type T;
}

impl Lt<'missing> for () { // { dg-error ".E0261." "" { target *-*-* } }
    type T = &'missing (); // { dg-error ".E0261." "" { target *-*-* } }
}

fn main() {
    let _: <() as Lt<'_>>::T = &();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

