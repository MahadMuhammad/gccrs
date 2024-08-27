fn main() {
    let outer_local:e_outer<&str, { let inner_local:e_inner<&str, }
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
// { dg-error "" "" { target *-*-* } .-5 }
}
// { dg-error "" "" { target *-*-* } .-1 }

