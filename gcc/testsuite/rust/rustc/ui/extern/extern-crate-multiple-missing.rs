// If multiple `extern crate` resolutions fail each of them should produce an error
extern crate bar; // { dg-error ".E0463." "" { target *-*-* } }
extern crate foo; // { dg-error ".E0463." "" { target *-*-* } }

fn main() {
    // If the crate name introduced by `extern crate` failed to resolve then subsequent
    // derived paths do not emit additional errors
    foo::something();
    bar::something();
}

