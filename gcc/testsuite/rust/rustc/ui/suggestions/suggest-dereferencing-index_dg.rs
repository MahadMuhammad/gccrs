//@ run-rustfix
#![allow(unused_variables)]

fn main() {
    let i: &usize = &1;
    let one_item_please: i32 = [1, 2, 3][i]; // { dg-error ".E0277." "" { target *-*-* } }
}

