//@ compile-flags: -Zdeduplicate-diagnostics=yes

use std::collections::HashMap;

fn main() {
    for _ in HashMap::new().iter().cloned() {} // { dg-error ".E0271." "" { target *-*-* } }
// { dg-error ".E0271." "" { target *-*-* } .-1 }
}

