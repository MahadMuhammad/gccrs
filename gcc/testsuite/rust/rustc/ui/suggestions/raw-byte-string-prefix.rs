// `br` and `rb` are easy to confuse; check that we issue a suggestion to help.

// { dg-additional-options "-frust-edition=2021" }

fn main() {
    rb"abc";
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
}

