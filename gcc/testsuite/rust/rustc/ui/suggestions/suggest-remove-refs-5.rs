//@ run-rustfix
fn main() {
    let v = &mut &mut Vec::<i32>::new();
    for _ in &mut &mut v {} // { dg-error ".E0277." "" { target *-*-* } }

    let v = &mut &mut [1u8];
    for _ in &mut v {} // { dg-error ".E0277." "" { target *-*-* } }
}

