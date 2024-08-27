trait Trait {
    fn baz(&self, _: Self) {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    fn bat(&self) -> Self {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
}

fn bar(x: &dyn Trait) {} // { dg-error ".E0038." "" { target *-*-* } }

trait Other: Sized {}

fn foo(x: &dyn Other) {} // { dg-error ".E0038." "" { target *-*-* } }

fn main() {}

