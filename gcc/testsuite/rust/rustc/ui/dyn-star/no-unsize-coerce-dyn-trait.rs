#![feature(dyn_star, trait_upcasting)]
// { dg-warning "" "" { target *-*-* } .-1 }

trait A: B {}
trait B {}
impl A for usize {}
impl B for usize {}

fn main() {
    let x: Box<dyn* A> = Box::new(1usize as dyn* A);
    let y: Box<dyn* B> = x;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

