//@ aux-build:found-staticlib.rs

//@ normalize-stderr-test: "\.nll/" -> "/"
//@ normalize-stderr-test: "\\\?\\" -> ""
//@ normalize-stderr-test: "(lib)?found_staticlib\.[a-z]+" -> "libfound_staticlib.somelib"

extern crate found_staticlib; // { dg-error ".E0462." "" { target *-*-* } }

fn main() {
    found_staticlib::foo();
}

