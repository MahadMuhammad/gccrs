#![feature(generic_arg_infer)]

struct All<'a, T, const N: usize> {
  v: &'a T,
}

struct BadInfer<_>;
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { dg-error ".E0392." "" { target *-*-* } .-2 }

fn all_fn<'a, T, const N: usize>() {}

fn bad_infer_fn<_>() {}
// { dg-error "" "" { target *-*-* } .-1 }


fn main() {
  let a: All<_, _, _>; // { dg-error ".E0107." "" { target *-*-* } }
  all_fn();
  let v: [u8; _];
  let v: [u8; 10] = [0; _];
}

