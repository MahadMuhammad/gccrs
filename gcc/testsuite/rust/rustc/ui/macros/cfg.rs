fn main() {
    cfg!(); // { dg-error "" "" { target *-*-* } }
    cfg!(123); // { dg-error "" "" { target *-*-* } }
    cfg!(foo = 123); // { dg-error ".E0565." "" { target *-*-* } }
    cfg!(foo, bar); // { dg-error "" "" { target *-*-* } }
}

