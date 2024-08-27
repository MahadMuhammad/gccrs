// Check the if let guards don't force capture by value
//@ revisions: e2018 e2021
// { dg-additional-options "-frust-edition=2018" }
// { dg-additional-options "-frust-edition=2021" }

#![feature(if_let_guard)]
#![allow(irrefutable_let_patterns)]

fn if_let_ref_mut(mut value: Box<E>) {
    let f = |x: &E| {
        match &x {
            E::Number(_) if let E::Number(ref mut n) = *value => { }
            _ => {}
        }
    };
    let x = value;
// { dg-error "" "" { target *-*-* } .-1 }
    drop(f);
}

fn if_let_move(value: Box<E>) {
    let f = |x: &E| {
        match &x {
            E::Number(_) if let E::String(s) = *value => { }
            _ => {}
        }
    };
    let x = value;
// { dg-error "" "" { target *-*-* } .-1 }
}

enum E {
    String(String),
    Number(i32),
}

fn main() {}

