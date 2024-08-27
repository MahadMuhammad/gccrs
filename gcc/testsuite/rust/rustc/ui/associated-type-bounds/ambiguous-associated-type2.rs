trait Foo {
    type Item;
}
trait Bar<T> {
    type Item;
}
trait Baz: Foo + Bar<Self::Item> {}
// { dg-error ".E0391." "" { target *-*-* } .-1 }



fn main() {}

