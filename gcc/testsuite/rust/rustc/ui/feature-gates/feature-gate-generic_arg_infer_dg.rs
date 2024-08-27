//@ [feature] run-pass
//@ revisions: normal feature

#![cfg_attr(feature, feature(generic_arg_infer))]

fn foo<const N: usize>(_: [u8; N]) -> [u8; N] {
  [0; N]
}

fn bar() {
    let _x: [u8; 3] = [0; _];
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    let _y: [u8; _] = [0; 3];
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

fn main() {
    let _x = foo::<_>([1,2]);
// { dg-error "" "" { target *-*-* } .-1 }
    bar();
}

