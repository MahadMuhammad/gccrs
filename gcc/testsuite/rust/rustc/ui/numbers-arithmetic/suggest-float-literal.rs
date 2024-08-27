//@ run-rustfix

#![allow(dead_code)]

fn add_integer_to_f32(x: f32) -> f32 {
    x + 100 // { dg-error ".E0277." "" { target *-*-* } }
}

fn add_integer_to_f64(x: f64) -> f64 {
    x + 100 // { dg-error ".E0277." "" { target *-*-* } }
}

fn subtract_integer_from_f32(x: f32) -> f32 {
    x - 100 // { dg-error ".E0277." "" { target *-*-* } }
}

fn subtract_integer_from_f64(x: f64) -> f64 {
    x - 100 // { dg-error ".E0277." "" { target *-*-* } }
}

fn multiply_f32_by_integer(x: f32) -> f32 {
    x * 100 // { dg-error ".E0277." "" { target *-*-* } }
}

fn multiply_f64_by_integer(x: f64) -> f64 {
    x * 100 // { dg-error ".E0277." "" { target *-*-* } }
}

fn divide_f32_by_integer(x: f32) -> f32 {
    x / 100 // { dg-error ".E0277." "" { target *-*-* } }
}

fn divide_f64_by_integer(x: f64) -> f64 {
    x / 100 // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {}

