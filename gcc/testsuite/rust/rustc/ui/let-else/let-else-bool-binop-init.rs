//@ run-rustfix



fn main() {
    let true = true && false else { return }; // { dg-error "" "" { target *-*-* } }
    let true = true || false else { return }; // { dg-error "" "" { target *-*-* } }
}

