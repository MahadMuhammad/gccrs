#[diagnostic::on_unimplemented(message = "Foo", label = "Bar", note = "Baz")]
trait Foo {}

fn takes_foo(_: impl Foo) {}

fn main() {
    takes_foo(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

