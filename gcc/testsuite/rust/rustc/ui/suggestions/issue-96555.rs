// { dg-additional-options "-frust-edition=2018" }

async fn f() {
    m::f1().await; // { dg-error ".E0277." "" { target *-*-* } }
    m::f2().await; // { dg-error ".E0277." "" { target *-*-* } }
    m::f3().await; // { dg-error ".E0277." "" { target *-*-* } }
}

mod m {
    pub fn f1() {}

    pub(crate) fn f2() {}

    pub
    fn
    f3() {}
}

fn main() {}

