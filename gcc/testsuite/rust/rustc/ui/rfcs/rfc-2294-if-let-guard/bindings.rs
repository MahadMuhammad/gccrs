#![feature(if_let_guard)]

fn main() {
    match Some(None) {
        Some(x) if let Some(y) = x => (x, y),
        _ => y, // { dg-error ".E0425." "" { target *-*-* } }
    }
    y // { dg-error ".E0425." "" { target *-*-* } }
}

