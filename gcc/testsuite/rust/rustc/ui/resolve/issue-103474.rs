struct S {}
impl S {
    fn first(&self) {}

    fn second(&self) {
        first()
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }

    fn third(&self) {
        no_method_err()
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }
}

// https://github.com/rust-lang/rust/pull/103531#discussion_r1004728080
struct Foo {
    i: i32,
}

impl Foo {
    fn needs_self() {
        this.i
// { dg-error ".E0425." "" { target *-*-* } .-1 }
    }
}

fn main() {}

