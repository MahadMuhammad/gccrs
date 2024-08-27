// { dg-additional-options "-frust-edition=2018" }
//@ aux-crate:issue_80074=issue-80074-macro.rs
//@ aux-crate:issue_80074_2=issue-80074-macro-2.rs

#[macro_use]
extern crate issue_80074;

#[macro_use(m)]
extern crate issue_80074_2;
// { dg-error ".E0469." "" { target *-*-* } .-2 }

fn main() {
    foo!();
// { dg-warning "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
    bar!();
// { dg-error "" "" { target *-*-* } .-1 }
    m!();
// { dg-error "" "" { target *-*-* } .-1 }
}

