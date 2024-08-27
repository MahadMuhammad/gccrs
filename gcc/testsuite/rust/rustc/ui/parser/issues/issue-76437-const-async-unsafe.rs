// { dg-additional-options "-frust-edition=2018" }

mod t {
    const async unsafe pub fn t() {}
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

