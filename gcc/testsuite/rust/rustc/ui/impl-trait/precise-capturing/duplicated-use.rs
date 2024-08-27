//@ revisions: real pre_expansion
//@[pre_expansion] check-pass

#[cfg(real)]
fn hello<'a>() -> impl Sized + use<'a> + use<'a> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

