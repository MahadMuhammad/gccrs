fn main() {

    let f;
    let g;

    g = f;
    f = Box::new(g);
// { dg-error ".E0275." "" { target *-*-* } .-1 }
}

