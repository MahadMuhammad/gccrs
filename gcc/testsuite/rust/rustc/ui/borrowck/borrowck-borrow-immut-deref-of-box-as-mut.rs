struct A;

impl A {
    fn foo(&mut self) {
    }
}



pub fn main() {
    let a: Box<_> = Box::new(A);
    a.foo();
// { dg-error ".E0596." "" { target *-*-* } .-1 }
}

