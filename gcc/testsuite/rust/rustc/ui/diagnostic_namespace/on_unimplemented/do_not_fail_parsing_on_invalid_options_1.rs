#[diagnostic::on_unimplemented(unsupported = "foo")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait Foo {}

#[diagnostic::on_unimplemented(message = "Baz")]
// { dg-warning "" "" { target *-*-* } .-1 }
struct Bar {}

#[diagnostic::on_unimplemented(message = "Boom", unsupported = "Bar")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait Baz {}

#[diagnostic::on_unimplemented(message = "Boom", on(_Self = "i32", message = "whatever"))]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait Boom {}

#[diagnostic::on_unimplemented = "boom"]
// { dg-warning "" "" { target *-*-* } .-1 }
trait Doom {}

#[diagnostic::on_unimplemented]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait Whatever {}

#[diagnostic::on_unimplemented(message = "{DoesNotExist}")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait Test {}

fn take_foo(_: impl Foo) {}
fn take_baz(_: impl Baz) {}
fn take_boom(_: impl Boom) {}
fn take_whatever(_: impl Whatever) {}
fn take_test(_: impl Test) {}

fn main() {
    take_foo(1_i32);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    take_baz(1_i32);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    take_boom(1_i32);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    take_whatever(1_i32);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    take_test(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

