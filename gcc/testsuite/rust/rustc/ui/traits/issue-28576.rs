pub trait Foo<RHS=Self> {
    type Assoc;
}

pub trait Bar: Foo<Assoc=()> {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    fn new(&self, b: &
           dyn Bar // { dg-error ".E0038." "" { target *-*-* } }
              <Assoc=()>
    );
}

fn main() {}

