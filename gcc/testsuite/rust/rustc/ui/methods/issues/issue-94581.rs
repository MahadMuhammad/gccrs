//@ run-rustfix
fn get_slice() -> &'static [i32] {
    &[1, 2, 3, 4]
}

fn main() {
    let _sqsum: i32 = get_slice().map(|i| i * i).sum(); // { dg-error ".E0599." "" { target *-*-* } }
}

