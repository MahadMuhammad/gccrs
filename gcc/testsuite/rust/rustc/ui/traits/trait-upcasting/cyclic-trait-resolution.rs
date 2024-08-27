trait A: B + A {}
// { dg-error ".E0391." "" { target *-*-* } .-1 }
// { dg-error ".E0391." "" { target *-*-* } .-2 }

trait B {}

impl A for () {}

impl B for () {}

fn main() {
    let a: Box<dyn A> = Box::new(());
    let _b: Box<dyn B> = a;
}

