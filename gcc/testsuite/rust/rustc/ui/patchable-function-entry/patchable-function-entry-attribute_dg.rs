#![feature(patchable_function_entry)]
fn main() {}

#[patchable_function_entry(prefix_nops = 256, entry_nops = 0)]// { dg-error "" "" { target *-*-* } }
pub fn too_high_pnops() {}

#[patchable_function_entry(prefix_nops = "stringvalue", entry_nops = 0)]// { dg-error "" "" { target *-*-* } }
pub fn non_int_nop() {}

#[patchable_function_entry]// { dg-error "" "" { target *-*-* } }
pub fn malformed_attribute() {}

#[patchable_function_entry(prefix_nops = 10, something = 0)]// { dg-error "" "" { target *-*-* } }
pub fn unexpected_parameter_name() {}

#[patchable_function_entry()]// { dg-error "" "" { target *-*-* } }
pub fn no_parameters_given() {}

