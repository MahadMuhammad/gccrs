struct Foo {
    bar: String,
}

impl Foo {
    pub fn new(bar: impl ToString) -> Self {
        Self {
            bar: bar.into(), // { dg-error ".E0277." "" { target *-*-* } }
        }
    }
}

fn main() {}

