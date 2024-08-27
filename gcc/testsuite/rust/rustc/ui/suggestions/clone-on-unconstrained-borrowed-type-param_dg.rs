//@ run-rustfix
fn wat<T>(t: &T) -> T {
    t.clone() // { dg-error ".E0308." "" { target *-*-* } }
}

struct Foo;

fn wut(t: &Foo) -> Foo {
    t.clone() // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {
    wat(&42);
    wut(&Foo);
}

