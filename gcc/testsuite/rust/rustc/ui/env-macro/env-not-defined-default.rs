fn main() {
    env!("CARGO__HOPEFULLY_NOT_DEFINED__");
// { dg-error "" "" { target *-*-* } .-1 }
}

