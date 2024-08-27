#![deny(unused)]
fn foo(xyza: &str) {
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = "{xyza}";
}

fn foo3(xyza: &str) {
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = "aaa{xyza}bbb";
}

fn main() {
  foo("x");
  foo3("xx");
}

