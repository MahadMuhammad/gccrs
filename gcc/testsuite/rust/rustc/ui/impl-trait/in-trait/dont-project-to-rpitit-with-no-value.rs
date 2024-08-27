trait MyTrait {
    fn foo(&self) -> impl Sized;
    fn bar(&self) -> impl Sized;
}

impl MyTrait for i32 {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
    fn bar(&self) -> impl Sized {
        self.foo()
    }
}

fn main() {}

