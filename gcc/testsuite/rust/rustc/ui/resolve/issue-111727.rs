// { dg-additional-options "-frust-edition= 2021" }

fn main() {
    std::any::Any::create();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-error ".E0599." "" { target *-*-* } .-2 }
}

