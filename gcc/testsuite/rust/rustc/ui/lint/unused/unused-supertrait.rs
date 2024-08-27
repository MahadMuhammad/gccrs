#![deny(unused_must_use)]

fn it() -> impl ExactSizeIterator<Item = ()> {
    let x: Box<dyn ExactSizeIterator<Item = ()>> = todo!();
    x
}

fn main() {
    it();
// { dg-error "" "" { target *-*-* } .-1 }
}

