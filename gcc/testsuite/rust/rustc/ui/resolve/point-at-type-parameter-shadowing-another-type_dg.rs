trait Foo<T> {
    fn foo(&self, name: T) -> usize;
}

struct Bar {
    baz: Baz,
}

struct Baz {
    num: usize,
}

impl<Baz> Foo<Baz> for Bar {
    fn foo(&self, _name: Baz) -> usize {
        match self.baz {
            Baz { num } => num, // { dg-error ".E0574." "" { target *-*-* } }
        }
    }
}

fn main() {}

