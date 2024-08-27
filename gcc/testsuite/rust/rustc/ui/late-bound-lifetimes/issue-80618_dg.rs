fn foo<'a>(x: &'a str) -> &'a str {
    x
}

fn main() {
    let _ = foo::<'static>;
// { dg-error ".E0794." "" { target *-*-* } .-1 }
}

