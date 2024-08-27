fn main() {}

extern "C" { // { dg-note "" "" { target *-*-* } }
    pub pub fn foo();
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
} // { dg-note "" "" { target *-*-* } }

