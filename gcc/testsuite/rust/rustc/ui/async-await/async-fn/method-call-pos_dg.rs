// { dg-additional-options "-frust-edition=2018" }

fn main() {
    <_ as async Fn()>(|| async {});
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
}

