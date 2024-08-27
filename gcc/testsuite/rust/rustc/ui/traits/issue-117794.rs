trait Foo {}

trait T {
    fn a(&self) -> impl Foo {
        self.b(|| 0)
// { dg-error ".E0599." "" { target *-*-* } .-1 }
    }
}

fn main() {}

