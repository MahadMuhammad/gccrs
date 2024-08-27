// Parsing of range patterns

fn main() {
    let 10 ..= 10 + 3 = 12;
// { dg-error "" "" { target *-*-* } .-1 }

    let 10 - 3 ..= 10 = 8;
// { dg-error "" "" { target *-*-* } .-1 }
}

