fn foo(a: &Option<String>, b: &Option<String>) {
    match (a, b) {
// { dg-error ".E0507." "" { target *-*-* } .-1 }
        (None, &c) => &c.unwrap(),
        (&Some(ref c), _) => c,
    };
}

fn main() {}

