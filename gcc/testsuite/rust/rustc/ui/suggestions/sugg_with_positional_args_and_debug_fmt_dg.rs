// When build the suggesttion take in consideration the `:?`
// https://github.com/rust-lang/rust/issues/100648
#![deny(warnings)]

fn main () {
    println!("hello {:?}", world = "world");
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { suggestion "" "" { target *-*-* } .-3 }
}

