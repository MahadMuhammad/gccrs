fn main() {}

fn attr_in_guard() {
    match None::<u32> {
        Some(!) // { dg-error ".E0658." "" { target *-*-* } }
// { dg-error ".E0658." "" { target *-*-* } .-1 }
            if #[deny(unused_mut)] // { dg-error ".E0658." "" { target *-*-* } }
            false // { dg-error "" "" { target *-*-* } }
    }
    match false {} // { dg-error ".E0004." "" { target *-*-* } }
}

