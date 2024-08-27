// When a SINGLE-character string literal is used where a char should be,
// suggest changing to single quotes.

// Testing both single-byte and multi-byte characters, as we should handle both.

//@ run-rustfix

fn main() {
    let _: char = "a"; // { dg-error ".E0308." "" { target *-*-* } }
    let _: char = "人"; // { dg-error ".E0308." "" { target *-*-* } }
    let _: char = "'"; // { dg-error ".E0308." "" { target *-*-* } }
}

// regression test for https://github.com/rust-lang/rust/issues/109586
#[allow(dead_code)]
fn convert_c_to_str(c: char) {
    match c {
        "A" => {} // { dg-error ".E0308." "" { target *-*-* } }
        _ => {}
    }
}

