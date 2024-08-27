fn foo() {}
fn main() {
    foo(;
    foo(;
} // { dg-error "" "" { target *-*-* } }

