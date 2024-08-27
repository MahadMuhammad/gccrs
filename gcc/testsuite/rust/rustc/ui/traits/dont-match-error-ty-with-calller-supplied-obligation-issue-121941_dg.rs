fn function<T: PartialEq>() {
    foo == 2; // { dg-error ".E0425." "" { target *-*-* } }
}

fn main() {}

