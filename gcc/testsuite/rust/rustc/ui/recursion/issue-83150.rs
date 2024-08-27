// { dg-error "" "" { target *-*-* } }
//@ build-fail
//@ compile-flags: -Copt-level=0
//@ normalize-stderr-test: "long-type-\d+" -> "long-type-hash"
//@ ignore-compare-mode-next-solver (hangs)

fn main() {
    let mut iter = 0u8..1;
    func(&mut iter)
}

fn func<T: Iterator<Item = u8>>(iter: &mut T) {
// { dg-warning "" "" { target *-*-* } .-1 }
    func(&mut iter.map(|x| x + 1))
}

