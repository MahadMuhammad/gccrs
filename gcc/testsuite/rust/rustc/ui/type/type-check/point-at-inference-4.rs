struct S<A, B>(Option<(A, B)>);

impl<A, B> S<A, B> {
    fn infer(&self, a: A, b: B) {}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
}

fn main() {
    let s = S(None);
    s.infer(0i32);
// { dg-error ".E0061." "" { target *-*-* } .-1 }
// { dg-note ".E0061." "" { target *-*-* } .-2 }
// { dg-note ".E0061." "" { target *-*-* } .-3 }
// { dg-note ".E0061." "" { target *-*-* } .-4 }
// { help ".E0061." "" { target *-*-* } .-5 }
// { help ".E0061." "" { target *-*-* } .-6 }
    let t: S<u32, _> = s;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
// { dg-note ".E0308." "" { target *-*-* } .-4 }
}

