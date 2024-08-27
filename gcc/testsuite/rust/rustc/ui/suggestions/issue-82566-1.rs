struct T1<const X1: usize>;
struct T2<const X1: usize, const X2: usize>;
struct T3<const X1: usize, const X2: usize, const X3: usize>;

impl T1<1> {
    const C: () = ();
}

impl T2<1, 2> {
    const C: () = ();
}

impl T3<1, 2, 3> {
    const C: () = ();
}

fn main() {
    T1<1>::C; // { dg-error "" "" { target *-*-* } }
    T2<1, 2>::C; // { dg-error "" "" { target *-*-* } }
    T3<1, 2, 3>::C; // { dg-error "" "" { target *-*-* } }
}

