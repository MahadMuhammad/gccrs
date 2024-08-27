fn main() {
    panic!(std::default::Default::default());
// { dg-error ".E0283." "" { target *-*-* } .-1 }
}

