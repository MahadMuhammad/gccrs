//@ unset-rustc-env:oopsie
//@ unset-rustc-env:a""a

env![r#"oopsie"#];
// { dg-error "" "" { target *-*-* } .-1 }

env![r#"a""a"#];
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

