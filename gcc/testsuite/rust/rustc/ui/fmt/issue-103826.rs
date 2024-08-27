fn main() {
    format!("{\x7D");
// { dg-error "" "" { target *-*-* } .-1 }
    format!("\x7B\x7D");
// { dg-error "" "" { target *-*-* } .-1 }
    format!("{\x7D {\x7D");
// { dg-error "" "" { target *-*-* } .-1 }
}

