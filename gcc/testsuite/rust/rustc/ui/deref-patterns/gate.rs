// gate-test-string_deref_patterns
fn main() {
    match String::new() {
        "" | _ => {}
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

