type Foo = impl Send;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

struct A;

const VALUE: Foo = value();
// { dg-error ".E0425." "" { target *-*-* } .-1 }

fn test() {
    match VALUE {
        0 | 0 => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
        _ => (),
    }
}

fn main() {}

