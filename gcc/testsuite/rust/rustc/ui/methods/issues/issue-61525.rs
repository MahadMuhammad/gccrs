pub trait Example {
    fn query<Q>(self, q: Q);
}

impl Example for i32 {
    fn query<Q>(self, _: Q) {
        unimplemented!()
    }
}

mod nested {
    use super::Example;
    fn example() {
        1.query::<dyn ToString>("")
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
    }
}

fn main() {}

