// #91594: This used to ICE.

trait Component<M> {
    type Interface;
}
trait HasComponent<I> {}

struct Foo;

impl HasComponent<<Foo as Component<Foo>>::Interface> for Foo {}
// { dg-error ".E0277." "" { target *-*-* } .-1 }

impl<M: HasComponent<()>> Component<M> for Foo {
    type Interface = u8;
}

fn main() {}

