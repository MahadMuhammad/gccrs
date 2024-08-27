trait Extend {
    fn extend<'a: 'a>(_: &'a str) -> (impl Sized + 'a, &'static str);
}

impl Extend for () {
    fn extend<'a: 'a>(s: &'a str) -> (Option<&'static &'a ()>, &'static str)
// { dg-error ".E0491." "" { target *-*-* } .-1 }
    where
        'a: 'static,
    {
        (None, s)
    }
}

// This indirection is not necessary for reproduction,
// but it makes this test future-proof against #114936.
fn extend<T: Extend>(s: &str) -> &'static str {
    <T as Extend>::extend(s).1
}

fn main() {
    let use_after_free = extend::<()>(&String::from("temporary"));
    println!("{}", use_after_free);
}

