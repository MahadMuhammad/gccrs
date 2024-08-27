const fn hey() -> usize {
    panic!(123); // { dg-error "" "" { target *-*-* } }
}

fn main() {
    let _: [u8; hey()] = todo!();
}

