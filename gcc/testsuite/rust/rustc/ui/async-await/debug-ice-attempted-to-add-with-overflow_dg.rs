//@ check-fail
// { dg-additional-options "-frust-edition=2021" }

// test for issue-114912 - debug ice: attempted to add with overflow

async fn main() {
// { dg-error ".E0752." "" { target *-*-* } .-1 }
    [0usize; 0xffff_ffff_ffff_ffff].await;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

