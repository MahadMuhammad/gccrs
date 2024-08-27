#![feature(rustc_attrs)]

#[rustc_dummy = stringify!(a)] // OK
macro_rules! bar {
    () => {};
}

// FIXME?: `bar` here expands before `stringify` has a chance to expand.
// `#[rustc_dummy = ...]` is validated and dropped during expansion of `bar`,
// the "attribute value must be a literal" error comes from the validation.
#[rustc_dummy = stringify!(b)] // { dg-error "" "" { target *-*-* } }
bar!();

fn main() {}

