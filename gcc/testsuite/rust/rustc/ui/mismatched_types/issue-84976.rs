/* Checks whether primitive type names are formatted correctly in the
 * error messages about mismatched types (#84976).
 */

fn foo(length: &u32) -> i32 {
    0
}

fn bar(length: &f32) -> f64 {
    0.0
}

fn main() {
    let mut length = 0;
    length = { foo(&length) };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    length = foo(&length);
// { dg-error ".E0308." "" { target *-*-* } .-1 }

    let mut float_length = 0.0;
    float_length = { bar(&float_length) };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    float_length = bar(&float_length);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

