pub trait Foo {}

struct Bar;
struct Baz;

impl Foo for Bar { }
impl Foo for Baz { }

fn not_all_paths(a: &str) -> u32 { // { dg-error ".E0308." "" { target *-*-* } }
    match a {
        "baz" => 0,
        _ => 1,
    };
}

fn right(b: &str) -> Box<dyn Foo> {
    match b {
        "baz" => Box::new(Baz),
        _ => Box::new(Bar),
    }
}

fn wrong(c: &str) -> Box<dyn Foo> { // { dg-error ".E0308." "" { target *-*-* } }
    match c {
        "baz" => Box::new(Baz),
        _ => Box::new(Bar), // { dg-error ".E0308." "" { target *-*-* } }
    };
}

fn main() {}

