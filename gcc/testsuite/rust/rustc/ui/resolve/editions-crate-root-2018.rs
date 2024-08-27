// { dg-additional-options "-frust-edition=2018" }

mod inner {
    fn global_inner(_: ::nonexistant::Foo) {
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    }
    fn crate_inner(_: crate::nonexistant::Foo) {
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    }

    fn bare_global(_: ::nonexistant) {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    }
    fn bare_crate(_: crate::nonexistant) {
// { dg-error ".E0412." "" { target *-*-* } .-1 }
    }
}

fn main() {

}

