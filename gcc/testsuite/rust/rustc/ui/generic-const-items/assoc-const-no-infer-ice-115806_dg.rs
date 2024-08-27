// ICE: assertion failed: !value.has_infer()
// issue: rust-lang/rust#115806
#![feature(associated_const_equality)]
#![allow(incomplete_features)]

pub struct NoPin;

impl<TA> Pins<TA> for NoPin {}

pub trait PinA<PER> {
    const A: &'static () = &();
}

pub trait Pins<USART> {}

impl<USART, T> Pins<USART> for T where T: PinA<USART, A = { &() }> {}
// { dg-error ".E0119." "" { target *-*-* } .-1 }

pub fn main() {}

