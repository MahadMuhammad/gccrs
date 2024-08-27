struct S {
    x: Box<isize>,
}



impl S {
    pub fn foo(self) -> isize {
        self.bar();
        return *self.x;  // { dg-error ".E0382." "" { target *-*-* } }
    }

    pub fn bar(self) {}
}

fn main() {
    let x = S { x: 1.into() };
    println!("{}", x.foo());
}

