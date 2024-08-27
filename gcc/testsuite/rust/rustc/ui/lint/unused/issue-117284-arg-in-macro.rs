#![deny(unused_variables)]
macro_rules! make_var {
    ($struct:ident, $var:ident) => {
        let $var = $struct.$var;
    };
}

#[allow(unused)]
struct MyStruct {
    var: i32,
}

fn main() {
    let s = MyStruct { var: 42 };
    make_var!(s, var); // { dg-error "" "" { target *-*-* } }
    let a = 1; // { dg-error "" "" { target *-*-* } }
}

