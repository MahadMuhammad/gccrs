#![deny(map_unit_fn)]

fn foo(items: &mut Vec<u8>) {
    items.sort();
}

fn main() {
    let mut x: Vec<Vec<u8>> = vec![vec![0, 2, 1], vec![5, 4, 3]];
    x.iter_mut().map(foo);
// { dg-error "" "" { target *-*-* } .-1 }
    x.iter_mut().map(|items| {
// { dg-error "" "" { target *-*-* } .-1 }
        items.sort();
    });
    let f = |items: &mut Vec<u8>| {
        items.sort();
    };
    x.iter_mut().map(f);
// { dg-error "" "" { target *-*-* } .-1 }
}

