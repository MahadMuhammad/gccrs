// This file has unexpected closing delimiter,

fn func(o: Option<u32>) {
    match o {
        Some(_x) => {}   // Extra '}'
            let _ = if true {};
        }
        None => {}
    }
} // { dg-error "" "" { target *-*-* } }

fn main() {}

