#![feature(if_let_guard)]

enum VecWrapper { A(Vec<i32>) }

fn if_guard(x: VecWrapper) -> usize {
    match x {
        VecWrapper::A(v) if { drop(v); false } => 1,
// { dg-error ".E0507." "" { target *-*-* } .-1 }
        VecWrapper::A(v) => v.len()
    }
}

fn if_let_guard(x: VecWrapper) -> usize {
    match x {
        VecWrapper::A(v) if let Some(()) = { drop(v); None } => 1,
// { dg-error ".E0507." "" { target *-*-* } .-1 }
        VecWrapper::A(v) => v.len()
    }
}

fn main() {
    if_guard(VecWrapper::A(vec![107]));
    if_let_guard(VecWrapper::A(vec![107]));
}

