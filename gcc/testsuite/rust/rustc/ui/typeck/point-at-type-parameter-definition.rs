trait Trait {
    fn do_stuff(&self);
}

struct Hello;

impl Hello {
    fn method(&self) {}
}

impl<Hello> Trait for Vec<Hello> {
    fn do_stuff(&self) {
        self[0].method(); // { dg-error ".E0599." "" { target *-*-* } }
    }
}

fn main() {}

