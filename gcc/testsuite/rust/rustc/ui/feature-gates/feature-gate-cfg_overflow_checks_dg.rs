#![crate_type = "lib"]

#[cfg(overflow_checks)] // { dg-error ".E0658." "" { target *-*-* } }
pub fn cast(v: i64)->u32{
    todo!()
}

