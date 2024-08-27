struct S {
    a: u32,
}

fn main() {
    let s1 = S { a: 1 };

    let _ = || {
        let s2 = Oops { a: 2, ..s1 };
// { dg-error ".E0422." "" { target *-*-* } .-1 }
    };
}

