struct Foo;

trait Bar {}

impl Bar for Foo {}

fn needs_bar<T: Bar>(_: T) {}

fn blah(f: fn() -> Foo) {
    needs_bar(f);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { help ".E0277." "" { target *-*-* } .-2 }
}

fn main() {}

