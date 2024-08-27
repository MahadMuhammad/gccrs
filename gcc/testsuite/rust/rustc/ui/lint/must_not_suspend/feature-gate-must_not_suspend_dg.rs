// { dg-additional-options "-frust-edition=2018" }

#[must_not_suspend = "You gotta use Umm's, ya know?"] // { dg-error ".E0658." "" { target *-*-* } }
struct Umm {
    _i: i64
}

fn main() {
}

