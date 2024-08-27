//@ compile-flags: --force-warn while_true
//@ compile-flags: --force-warn unused_variables
//@ compile-flags: --force-warn unused_mut
//@ check-pass

fn expect_early_pass_lint(terminate: bool) {
    #[expect(while_true)]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    while !terminate {
        println!("Do you know what a spin lock is?")
    }
}

#[expect(unused_variables, reason="<this should fail and display this reason>")]
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
fn check_specific_lint() {
    let _x = 2;
}

#[expect(unused)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn check_multiple_lints_with_lint_group() {
    let fox_name = "Sir Nibbles";

    let what_does_the_fox_say = "*ding* *deng* *dung*";

    println!("The fox says: {what_does_the_fox_say}");
    println!("~ {fox_name}")
}


#[expect(unused)]
// { dg-warning "" "" { target *-*-* } .-1 }
fn check_overridden_expectation_lint_level() {
    #[allow(unused_variables)]
    let this_should_not_fulfill_the_expectation = "maybe";
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
}

fn main() {
    check_multiple_lints_with_lint_group();
    check_overridden_expectation_lint_level();
}

