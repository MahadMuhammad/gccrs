fn main() {
    const FOO = "hello" + 1; // { dg-error ".E0369." "" { target *-*-* } }
// { dg-error ".E0369." "" { target *-*-* } .-1 }
// { dg-error ".E0369." "" { target *-*-* } .-2 }
    println!("{}", FOO);
}

