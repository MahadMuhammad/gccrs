// Test that we suggest specifying the generic argument of `channel`
// instead of the return type of that function, which is a lot more
// complex.
use std::sync::mpsc::channel;

fn no_tuple() {
    let _data =
        channel(); // { dg-error ".E0282." "" { target *-*-* } }
}

fn tuple() {
    let (_sender, _receiver) =
        channel(); // { dg-error ".E0282." "" { target *-*-* } }
}

fn main() {
    no_tuple();
    tuple();
}

