// { dg-additional-options "-frust-edition=2018" }

mod t {
    unsafe pub(crate) fn t() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

