trait Modify {
    fn modify(&mut self) ;
}

impl<T> Modify for T  {
    fn modify(&mut self)  {}
}

trait Foo {
    fn mute(&mut self) {
        self.modify(); // { dg-error ".E0596." "" { target *-*-* } }
    }
}

fn main() {}

