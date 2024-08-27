#[deny(unused)]
fn main() {
    let arr = [0; 10];
    let _ = arr[(0)]; // { dg-error "" "" { target *-*-* } }
    let _ = arr[{0}]; // { dg-error "" "" { target *-*-* } }
    let _ = arr[1 + (0)];
    let _ = arr[{ let x = 0; x }];
}

