//@ check-fail

// First format below would cause a panic, second would generate error with incorrect span

fn main() {
    let _ = format!("→{}→\n");
// { dg-error "" "" { target *-*-* } .-1 }
    let _ = format!("→{} \n");
// { dg-error "" "" { target *-*-* } .-1 }
}

