#![feature(fn_delegation)]
#![allow(incomplete_features)]

macro_rules! emit_self { () => { self } }
// { dg-error ".E0424." "" { target *-*-* } .-1 }
// { dg-error ".E0424." "" { target *-*-* } .-2 }

struct S;
impl S {
    fn method(self) {
        emit_self!();
    }
}

fn foo(arg: u8) {}
reuse foo as bar {
    emit_self!()
}

fn main() {}

