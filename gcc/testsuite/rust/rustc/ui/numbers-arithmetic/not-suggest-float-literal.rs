fn add_float_to_integer(x: u8) -> f32 {
    x + 100.0 // { dg-error ".E0277." "" { target *-*-* } }
}

fn add_str_to_float(x: f64) -> f64 {
    x + "foo" // { dg-error ".E0277." "" { target *-*-* } }
}

fn add_lvar_to_float(x: f64) -> f64 {
    let y = 3;
    x + y // { dg-error ".E0277." "" { target *-*-* } }
}

fn subtract_float_from_integer(x: u8) -> f32 {
    x - 100.0 // { dg-error ".E0277." "" { target *-*-* } }
}

fn subtract_str_from_f64(x: f64) -> f64 {
    x - "foo" // { dg-error ".E0277." "" { target *-*-* } }
}

fn subtract_lvar_from_f64(x: f64) -> f64 {
    let y = 3;
    x - y // { dg-error ".E0277." "" { target *-*-* } }
}

fn multiply_integer_by_float(x: u8) -> f32 {
    x * 100.0 // { dg-error ".E0277." "" { target *-*-* } }
}

fn multiply_f64_by_str(x: f64) -> f64 {
    x * "foo" // { dg-error ".E0277." "" { target *-*-* } }
}

fn multiply_f64_by_lvar(x: f64) -> f64 {
    let y = 3;
    x * y // { dg-error ".E0277." "" { target *-*-* } }
}

fn divide_integer_by_float(x: u8) -> u8 {
    x / 100.0 // { dg-error ".E0277." "" { target *-*-* } }
}

fn divide_f64_by_str(x: f64) -> f64 {
    x / "foo" // { dg-error ".E0277." "" { target *-*-* } }
}

fn divide_f64_by_lvar(x: f64) -> f64 {
    let y = 3;
    x / y // { dg-error ".E0277." "" { target *-*-* } }
}

fn main() {}

