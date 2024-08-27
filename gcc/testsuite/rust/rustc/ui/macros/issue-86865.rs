use std::fmt::Write;

fn main() {
    println!(b"foo");
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
    let mut s = String::new();
    write!(s, b"foo{}", "bar");
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

