// gate-test-const_closures

fn main() {
    (const || {})();
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

macro_rules! e {
    ($e:expr) => {}
}

e!((const || {}));
// { dg-error ".E0658." "" { target *-*-* } .-1 }

