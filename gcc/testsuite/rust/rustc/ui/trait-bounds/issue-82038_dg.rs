// Failed bound `bool: Foo` must not point at the `Self: Clone` line

trait Foo {
    fn my_method() where Self: Clone;
}

fn main() {
    <bool as Foo>::my_method(); // { dg-error ".E0277." "" { target *-*-* } }
}

