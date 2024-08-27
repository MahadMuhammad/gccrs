#[diagnostic::on_unimplemented(message = "{{Test } thing")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait ImportantTrait1 {}

#[diagnostic::on_unimplemented(message = "Test {}")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait ImportantTrait2 {}

#[diagnostic::on_unimplemented(message = "Test {1:}")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait ImportantTrait3 {}

#[diagnostic::on_unimplemented(message = "Test {Self:123}")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
trait ImportantTrait4 {}

#[diagnostic::on_unimplemented(message = "Test {Self:!}")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-warning "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
trait ImportantTrait5 {}

fn check_1(_: impl ImportantTrait1) {}
fn check_2(_: impl ImportantTrait2) {}
fn check_3(_: impl ImportantTrait3) {}
fn check_4(_: impl ImportantTrait4) {}
fn check_5(_: impl ImportantTrait5) {}

fn main() {
    check_1(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_2(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_3(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_4(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    check_5(());
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

