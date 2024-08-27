extern "C" {
    #[linkage = "extern_weak"] static foo: *mut isize;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

fn main() {}

