// { dg-additional-options "-frust-edition=2018" }
#[deny(bare_trait_objects)]

fn function(x: &SomeTrait, y: Box<SomeTrait>) {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-warning "" "" { target *-*-* } .-4 }
    let _x: &SomeTrait = todo!();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-warning "" "" { target *-*-* } .-2 }
}

trait SomeTrait {}

fn main() {}

