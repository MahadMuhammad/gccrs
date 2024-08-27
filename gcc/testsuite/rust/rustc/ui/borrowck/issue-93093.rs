// { dg-additional-options "-frust-edition=2018" }
struct S {
    foo: usize,
}
impl S {
    async fn bar(&self) { // { help "" "" { target *-*-* } }
// { suggestion "" "" { target *-*-* } .-2 }
        self.foo += 1; // { dg-error ".E0594." "" { target *-*-* } }
    }
}

fn main() {
    S { foo: 1 }.bar();
}

