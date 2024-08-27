trait Foo {
// { dg-error ".E0391." "" { target *-*-* } .-1 }
    type Context<'c>
    where
        Self: 'c;
}

impl Foo for Box<dyn Foo> {}

fn main() {}

