// Test scope annotations from `parent_label` parameter

#![feature(rustc_attrs)]

#[rustc_on_unimplemented(parent_label = "in this scope")]
trait Trait {}

struct Foo;

fn f<T: Trait>(x: T) {}

fn main() {
    let x = || {
        f(Foo {}); // { dg-error ".E0277." "" { target *-*-* } }
        let y = || {
            f(Foo {}); // { dg-error ".E0277." "" { target *-*-* } }
        };
    };

    {
        {
            f(Foo {}); // { dg-error ".E0277." "" { target *-*-* } }
        }
    }

    f(Foo {}); // { dg-error ".E0277." "" { target *-*-* } }
}

