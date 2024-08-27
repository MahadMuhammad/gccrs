// Test that we do some basic error correction in the tokeniser.

fn main() {
    foo(bar(;
}
// { dg-error "" "" { target *-*-* } .-1 }

fn foo(_: usize) {}

