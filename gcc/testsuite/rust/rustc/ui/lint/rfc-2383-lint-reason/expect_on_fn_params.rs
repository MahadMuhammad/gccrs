//@ check-pass

#[warn(unused_variables)]

/// This should catch the unused_variables lint and not emit anything
fn check_fulfilled_expectation(#[expect(unused_variables)] unused_value: u32) {}

fn check_unfulfilled_expectation(#[expect(unused_variables)] used_value: u32) {
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    println!("I use the value {used_value}");
}

fn main() {}

