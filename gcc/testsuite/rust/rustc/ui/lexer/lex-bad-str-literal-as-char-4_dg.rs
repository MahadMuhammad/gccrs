// { dg-additional-options "-frust-edition=2021" }
macro_rules! foo {
    () => {
        println!('hello world');
// { dg-error ".E0762." "" { target *-*-* } .-1 }
// { dg-error ".E0762." "" { target *-*-* } .-2 }
    }
}
fn main() {}

