use spam::*; // { dg-error ".E0432." "" { target *-*-* } }

fn main() {
    // Expect these to pass because the compiler knows there's a failed `*` import that might have
    // caused it.
    ham();
    eggs();
    // Even this case, as we might have expected `spam::foo` to exist.
    foo::bar();
}

