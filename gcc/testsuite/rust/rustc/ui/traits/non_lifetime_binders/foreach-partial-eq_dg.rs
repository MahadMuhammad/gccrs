#![feature(non_lifetime_binders)]
// { dg-warning "" "" { target *-*-* } .-1 }

fn auto_trait()
where
    for<T> T: PartialEq + PartialOrd,
{}

fn main() {
    auto_trait();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

