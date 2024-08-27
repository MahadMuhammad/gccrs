#![feature(let_chains)]

fn let_or_guard(x: Result<Option<i32>, ()>) {
    match x {
        Ok(opt) if let Some(4) = opt || false  => {}
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }
}

fn hiding_unsafe_mod(x: Result<Option<i32>, ()>) {
    match x {
        Ok(opt)
            if {
                unsafe mod a {};
// { dg-error "" "" { target *-*-* } .-1 }
                false
            } => {}
        _ => {}
    }
}

fn main() {}

