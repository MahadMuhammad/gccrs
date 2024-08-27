//@ compile-flags: -Zunstable-options

pub fn issue_111280() {
    struct_span_err(msg).emit(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

fn main() {}

