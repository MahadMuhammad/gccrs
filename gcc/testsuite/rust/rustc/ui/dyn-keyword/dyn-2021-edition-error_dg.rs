// { dg-additional-options "-frust-edition=2021" }

fn function(x: &SomeTrait, y: Box<SomeTrait>) {
// { dg-error ".E0782." "" { target *-*-* } .-1 }
// { dg-error ".E0782." "" { target *-*-* } .-2 }
    let _x: &SomeTrait = todo!();
// { dg-error ".E0782." "" { target *-*-* } .-1 }
}

trait SomeTrait {}

fn main() {}

