struct Foo<T>(T, T);

impl<T> Foo<T> {
    fn foo(&self) {
        match *self {
            Foo<T>(x, y) => { // { dg-error "" "" { target *-*-* } }
              println!("Goodbye, World!")
            }
        }
    }
}

fn main() {}

