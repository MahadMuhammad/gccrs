fn main() {
    let mut x // { dg-note "" "" { target *-*-* } }
        =
        2; // { dg-note "" "" { target *-*-* } }
    x = 5.0;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
}

