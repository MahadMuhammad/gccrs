fn foo<T>() {
    struct Foo {
        x: T, // { dg-error ".E0401." "" { target *-*-* } }
    }

    impl<T> Drop for Foo<T> {
// { dg-error ".E0107." "" { target *-*-* } .-1 }
        fn drop(&mut self) {}
    }
}
fn main() {}

