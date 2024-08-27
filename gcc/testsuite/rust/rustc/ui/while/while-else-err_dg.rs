fn main() {
    while false {
// { dg-note "" "" { target *-*-* } .-1 }
    } else {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };
}

