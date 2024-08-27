#![feature(associated_const_equality)]

pub enum Mode {
    Cool,
}

pub trait Parse {
    const MODE: Mode;
}

pub trait CoolStuff: Parse<MODE = Mode::Cool> {}
// { dg-error ".E0573." "" { target *-*-* } .-1 }
// { dg-error ".E0573." "" { target *-*-* } .-2 }
// { dg-error ".E0573." "" { target *-*-* } .-3 }
// { dg-error ".E0573." "" { target *-*-* } .-4 }

fn no_help() -> Mode::Cool {}
// { dg-error ".E0573." "" { target *-*-* } .-1 }

fn main() {}

