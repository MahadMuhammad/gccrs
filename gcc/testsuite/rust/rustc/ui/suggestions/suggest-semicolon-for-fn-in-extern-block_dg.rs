//@ run-rustfix

#[allow(dead_code)]

extern "C" {
  fn foo() // { dg-error "" "" { target *-*-* } }
}

fn main() {}

