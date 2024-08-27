// Don't panic when iterating through the `hir::Map::parent_iter` of an RPITIT.

pub trait Foo {
    fn demo() -> impl Foo
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    where
        String: Copy;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn main() {}

