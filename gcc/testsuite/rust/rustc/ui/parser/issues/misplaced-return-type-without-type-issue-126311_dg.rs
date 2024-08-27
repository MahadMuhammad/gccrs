fn foo<T>() where T: Default -> {
// { dg-error "" "" { target *-*-* } .-1 }
    0
}

fn main() {}

