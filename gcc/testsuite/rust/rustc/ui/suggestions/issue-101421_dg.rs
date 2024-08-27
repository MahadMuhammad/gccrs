pub trait Ice {
    fn f(&self, _: ());
}

impl Ice for () {
    fn f(&self, _: ()) {}
}

fn main() {
    ().f::<()>(());
// { dg-error ".E0107." "" { target *-*-* } .-1 }
}

