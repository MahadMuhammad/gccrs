trait Foo {
}



impl<T:Copy> Foo for T {
}

fn take_param<T:Foo>(foo: &T) { }

fn main() {
    let x: Box<_> = Box::new(3);
    take_param(&x);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

