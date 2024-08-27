#![feature(inline_const_pat)]

fn uwu() {}

fn main() {
    let x = [];
    match x[123] {
        const { uwu } => {}
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }
}

