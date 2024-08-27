// { dg-additional-options "-frust-edition= 2021" }

trait Has {
    fn has() {}
}

trait HasNot {}

fn main() {
    HasNot::has();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-error ".E0599." "" { target *-*-* } .-2 }
}

