fn infer_in_arg() {
    let x = |b: Vec<_>| {}; // { dg-error ".E0282." "" { target *-*-* } }
}

fn empty_pattern() {
    let x = |_| {}; // { dg-error ".E0282." "" { target *-*-* } }
}

fn infer_ty() {
    let x = |k: _| {}; // { dg-error ".E0282." "" { target *-*-* } }
}

fn ambig_return() {
    let x = || -> Vec<_> { Vec::new() }; // { dg-error ".E0282." "" { target *-*-* } }
}

fn main() {}

