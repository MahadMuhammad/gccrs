//@ run-rustfix
fn main() {
    println!('1 + 1');
// { dg-error ".E0762." "" { target *-*-* } .-1 }
// { dg-error ".E0762." "" { target *-*-* } .-2 }
}

