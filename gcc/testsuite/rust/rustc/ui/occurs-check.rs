fn main() {
    let f;
    f = Box::new(f);
// { dg-error ".E0275." "" { target *-*-* } .-1 }
}

