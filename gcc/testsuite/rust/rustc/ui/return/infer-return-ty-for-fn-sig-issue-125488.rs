//@ run-rustfix

#[allow(dead_code)]

fn main() {
    struct S<'a>(&'a ());

    fn f1(s: S<'_>) -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
        s
    }

    fn f2(s: S<'_>) -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
        let x = true;
        if x {
            s
        } else {
            s
        }
    }

    fn f3(s: S<'_>) -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
        return s;
    }

    fn f4(s: S<'_>) -> _ {
// { dg-error ".E0121." "" { target *-*-* } .-1 }
        let _x = 1;
        return s;
    }
}

