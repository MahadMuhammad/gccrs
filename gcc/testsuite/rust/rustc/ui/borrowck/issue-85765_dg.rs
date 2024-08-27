fn main() {
    let mut test = Vec::new();
    let rofl: &Vec<Vec<i32>> = &mut test;
// { help "" "" { target *-*-* } .-1 }
    rofl.push(Vec::new());
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { dg-note ".E0596." "" { target *-*-* } .-2 }

    let mut mutvar = 42;
    let r = &mutvar;
// { help "" "" { target *-*-* } .-1 }
    *r = 0;
// { dg-error ".E0594." "" { target *-*-* } .-1 }
// { dg-note ".E0594." "" { target *-*-* } .-2 }

    #[rustfmt::skip]
    let x: &usize = &mut{0};
// { help "" "" { target *-*-* } .-1 }
    *x = 1;
// { dg-error ".E0594." "" { target *-*-* } .-1 }
// { dg-note ".E0594." "" { target *-*-* } .-2 }

    #[rustfmt::skip]
    let y: &usize = &mut(0);
// { help "" "" { target *-*-* } .-1 }
    *y = 1;
// { dg-error ".E0594." "" { target *-*-* } .-1 }
// { dg-note ".E0594." "" { target *-*-* } .-2 }
}

