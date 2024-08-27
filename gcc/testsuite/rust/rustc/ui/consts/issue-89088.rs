// Regression test for the ICE described in #89088.

use std::borrow::Cow;

const FOO: &A = &A::Field(Cow::Borrowed("foo"));

#[derive(PartialEq, Eq)]
enum A {
    Field(Cow<'static, str>)
}

fn main() {
    let var = A::Field(Cow::Borrowed("bar"));

    match &var {
        FOO => todo!(), // { dg-error "" "" { target *-*-* } }
        _ => todo!()
    }
}

