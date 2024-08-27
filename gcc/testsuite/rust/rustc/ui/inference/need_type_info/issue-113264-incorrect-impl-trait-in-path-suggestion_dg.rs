trait T {}

struct S {}

impl S {
    fn owo(&self, _: Option<&impl T>) {}
}

fn main() {
    (S {}).owo(None)
// { dg-error ".E0283." "" { target *-*-* } .-1 }
}

