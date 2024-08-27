fn main() {
    let _ = for _ in 0..1 {
// { dg-note "" "" { target *-*-* } .-1 }
    } else {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };
}

