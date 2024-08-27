fn main() {
    let f = |3: isize| println!("hello");
// { dg-error ".E0005." "" { target *-*-* } .-1 }
// { dg-error ".E0005." "" { target *-*-* } .-2 }
    f(4);
}

