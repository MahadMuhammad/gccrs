//@ run-rustfix

fn two_type_params<A, B>(_: B) {}

fn main() {
    two_type_params::<String>(100); // { dg-error ".E0107." "" { target *-*-* } }
    two_type_params::<String, _>(100);
}

