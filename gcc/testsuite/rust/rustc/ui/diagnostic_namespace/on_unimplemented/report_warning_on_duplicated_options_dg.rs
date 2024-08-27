#[diagnostic::on_unimplemented(
    message = "first message",
    label = "first label",
    note = "custom note"
)]
#[diagnostic::on_unimplemented(
    message = "second message",
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    label = "second label",
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    note = "second note"
)]
trait Foo {}


fn takes_foo(_: impl Foo) {}

fn main() {
    takes_foo(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

