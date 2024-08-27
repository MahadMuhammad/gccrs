#![feature(inline_const_pat)]

fn main() {
    match () {
        const { (|| {})() } => {}
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
    }
}

