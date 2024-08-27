fn main() {
    let ok = r"ab\[c";
    let bad = "ab\[c";
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
}

