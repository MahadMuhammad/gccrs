struct Take(Take);
// { dg-error ".E0391." "" { target *-*-* } .-1 }
// { dg-error ".E0391." "" { target *-*-* } .-2 }

// check that we don't hang trying to find the tail of a recursive struct (#79437)
fn foo() -> Take {
    Take(loop {})
}

// mutually infinite structs
struct Foo { // { dg-error ".E0072." "" { target *-*-* } }
    x: Bar<Foo>,
}

struct Bar<T>([T; 1]);

fn main() {}

