fn test0() {
    let mut i = 0;
    let _ = i + i--; // { dg-error "" "" { target *-*-* } }
    // won't suggest since we can not handle the precedences
}

fn test1() {
    let mut i = 0;
    let _ = i-- + i--; // { dg-error "" "" { target *-*-* } }
}

fn test2() {
    let mut i = 0;
    let _ = --i + i--; // { dg-error "" "" { target *-*-* } }
}

fn test3() {
    let mut i = 0;
    let _ = i-- + --i; // { dg-error "" "" { target *-*-* } }
}

fn test4() {
    let mut i = 0;
    let _ = (1 + 2 + i)--; // { dg-error "" "" { target *-*-* } }
}

fn test5() {
    let mut i = 0;
    let _ = (i-- + 1) + 2; // { dg-error "" "" { target *-*-* } }
}

fn test6(){
    let i=10;
    while i != 0 {
        i--; // { dg-error "" "" { target *-*-* } }
    }
}

fn main() {}

