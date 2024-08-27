//@ compile-flags: -Zdeduplicate-diagnostics=yes

#[forbid(unused_variables)]
// { dg-note "" "" { target *-*-* } .-1 }
#[expect(unused_variables)]
// { dg-error ".E0453." "" { target *-*-* } .-1 }
// { dg-note ".E0453." "" { target *-*-* } .-2 }
fn expect_forbidden_lint_1() {}

#[forbid(while_true)]
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
#[expect(while_true)]
// { dg-error ".E0453." "" { target *-*-* } .-1 }
// { dg-note ".E0453." "" { target *-*-* } .-2 }
fn expect_forbidden_lint_2() {
    // This while loop will produce a `while_true` lint as the lint level
    // at this node is still `forbid` and the `while_true` check happens
    // before the compilation terminates due to `E0453`
    while true {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn main() {
    expect_forbidden_lint_1();
    expect_forbidden_lint_2();
}

