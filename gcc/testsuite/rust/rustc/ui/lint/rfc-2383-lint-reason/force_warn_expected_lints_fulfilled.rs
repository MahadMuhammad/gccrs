//@ compile-flags: --force-warn while_true
//@ compile-flags: --force-warn unused_variables
//@ compile-flags: --force-warn unused_mut
//@ check-pass

fn expect_early_pass_lint() {
    #[expect(while_true)]
    while true {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        println!("I never stop")
    }
}

#[expect(unused_variables, reason="<this should fail and display this reason>")]
fn check_specific_lint() {
    let x = 2;
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
}

#[expect(unused)]
fn check_multiple_lints_with_lint_group() {
    let fox_name = "Sir Nibbles";
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    let mut what_does_the_fox_say = "*ding* *deng* *dung*";
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }

    println!("The fox says: {what_does_the_fox_say}");
}

#[allow(unused_variables)]
fn check_expect_overrides_allow_lint_level() {
    #[expect(unused_variables)]
    let this_should_fulfill_the_expectation = "The `#[allow]` has no power here";
// { dg-warning "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn main() {}

