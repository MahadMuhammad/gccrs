// Checks that a sibling function (i.e. `foo`) cannot constrain
// an RPITIT from another function (`bar`).

trait Trait {
    fn foo();

    fn bar() -> impl Sized;
}

impl Trait for () {
    fn foo() {
        let _: String = Self::bar();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }

    fn bar() -> impl Sized {
        loop {}
    }
}

fn main() {}

