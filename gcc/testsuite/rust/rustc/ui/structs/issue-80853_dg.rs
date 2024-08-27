struct S;

fn repro_ref(thing: S) {
    thing(); // { dg-error ".E0618." "" { target *-*-* } }
}

fn main() {}

