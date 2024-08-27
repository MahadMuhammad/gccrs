struct S {
    x: i32,
}

fn test1() {
    let mut i = 0;
    i++; // { dg-error "" "" { target *-*-* } }
}

fn test2() {
    let s = S { x: 0 };
    s.x++; // { dg-error "" "" { target *-*-* } }
}

fn test3() {
    let mut i = 0;
    if i++ == 1 {} // { dg-error "" "" { target *-*-* } }
}

fn test4() {
    let mut i = 0;
    ++i; // { dg-error "" "" { target *-*-* } }
}

fn test5() {
    let mut i = 0;
    if ++i == 1 { } // { dg-error "" "" { target *-*-* } }
}

fn test6() {
    let mut i = 0;
    loop { break; }
    i++; // { dg-error "" "" { target *-*-* } }
    loop { break; }
    ++i;
}

fn test7() {
    let mut i = 0;
    loop { break; }
    ++i; // { dg-error "" "" { target *-*-* } }
}


fn main() {}

