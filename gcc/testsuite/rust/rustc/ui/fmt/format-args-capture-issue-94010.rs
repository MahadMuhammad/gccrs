fn main() {
    const FOO: i32 = 123;
    println!("{foo:X}");
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    println!("{:.foo$}", 0);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

