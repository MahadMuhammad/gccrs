#[diagnostic::on_unimplemented(
    if(Self = "()"),
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    message = "custom message",
    note = "custom note"
)]
#[diagnostic::on_unimplemented(message = "fallback!!")]
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
#[diagnostic::on_unimplemented(label = "fallback label")]
#[diagnostic::on_unimplemented(note = "fallback note")]
trait Foo {}

fn takes_foo(_: impl Foo) {}

fn main() {
    takes_foo(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

