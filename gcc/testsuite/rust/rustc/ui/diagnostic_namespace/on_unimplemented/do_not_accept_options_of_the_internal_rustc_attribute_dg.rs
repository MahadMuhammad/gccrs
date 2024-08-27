#[diagnostic::on_unimplemented(
    on(_Self = "&str"),
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    message = "trait has `{Self}` and `{T}` as params",
    label = "trait has `{Self}` and `{T}` as params",
    note  = "trait has `{Self}` and `{T}` as params",
    parent_label = "in this scope",
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    append_const_msg
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
)]
trait Foo<T> {}

#[diagnostic::on_unimplemented = "Message"]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait Bar {}

#[diagnostic::on_unimplemented(message = "Not allowed to apply it on a impl")]
// { dg-warning "" "" { target *-*-* } .-1 }
impl Bar for i32 {}

// cannot use special rustc_on_unimplement symbols
// in the format string
#[diagnostic::on_unimplemented(
    message = "{from_desugaring}{direct}{cause}{integral}{integer}",
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { dg-warning "" "" { target *-*-* } .-5 }
// { dg-warning "" "" { target *-*-* } .-6 }
// { dg-warning "" "" { target *-*-* } .-7 }
// { dg-warning "" "" { target *-*-* } .-8 }
// { dg-warning "" "" { target *-*-* } .-9 }
// { dg-warning "" "" { target *-*-* } .-10 }
    label = "{float}{_Self}{crate_local}{Trait}{ItemContext}"
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
// { dg-warning "" "" { target *-*-* } .-5 }
// { dg-warning "" "" { target *-*-* } .-6 }
// { dg-warning "" "" { target *-*-* } .-7 }
// { dg-warning "" "" { target *-*-* } .-8 }
// { dg-warning "" "" { target *-*-* } .-9 }
// { dg-warning "" "" { target *-*-* } .-10 }
)]
trait Baz {}

fn takes_foo(_: impl Foo<i32>) {}
fn takes_bar(_: impl Bar) {}
fn takes_baz(_: impl Baz) {}

fn main() {
    takes_foo(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    takes_bar(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    takes_baz(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

