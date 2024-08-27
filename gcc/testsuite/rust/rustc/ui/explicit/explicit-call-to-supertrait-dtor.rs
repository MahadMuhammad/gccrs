//@ run-rustfix

#![allow(dead_code)]
#![allow(dropping_references)]

struct Foo {
    x: isize
}

#[allow(drop_bounds)]
trait Bar: Drop {
    fn blah(&self);
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("kaboom");
    }
}

impl Bar for Foo {
    fn blah(&self) {
        self.drop();    // { dg-error ".E0040." "" { target *-*-* } }
    }
}

fn main() {
    let x = Foo { x: 3 };
    println!("{}", x.x);
}

