fn main() {
    let x = foo::Foo::default();
    if x.len {
// { dg-error ".E0616." "" { target *-*-* } .-1 }
        println!("foo");
    }
}

mod foo {
    #[derive(Default)]
    pub struct Foo {
        len: String,
    }

    impl Foo {
        pub fn len(&self) -> usize {
            42
        }
    }
}

