#![feature(control_flow_enum)]

use std::ops::ControlFlow;

fn result_to_result() -> Result<u64, u8> {
    Ok(Err(123_i32)?)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn option_to_result() -> Result<u64, String> {
    Some(3)?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    Ok(10)
}

fn control_flow_to_result() -> Result<u64, String> {
    Ok(ControlFlow::Break(123)?)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn result_to_option() -> Option<u16> {
    Some(Err("hello")?)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn control_flow_to_option() -> Option<u64> {
    Some(ControlFlow::Break(123)?)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn result_to_control_flow() -> ControlFlow<String> {
    ControlFlow::Continue(Err("hello")?)
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

fn option_to_control_flow() -> ControlFlow<u64> {
    Some(3)?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    ControlFlow::Break(10)
}

fn control_flow_to_control_flow() -> ControlFlow<i64> {
    ControlFlow::Break(4_u8)?;
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    ControlFlow::Continue(())
}

fn main() {}

