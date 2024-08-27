#![crate_type = "lib"]

pub fn ub_checks_are_enabled() -> bool {
    cfg!(ub_checks) // { dg-error ".E0658." "" { target *-*-* } }
}

