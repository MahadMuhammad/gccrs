//@ run-rustfix

// These are forbidden occurrences of label-break-value

#[allow(unused_unsafe)]
fn labeled_unsafe() {
    unsafe 'b: {} // { dg-error "" "" { target *-*-* } }
}

fn labeled_if() {
    if true 'b: {} // { dg-error "" "" { target *-*-* } }
}

fn labeled_else() {
    if true {} else 'b: {} // { dg-error "" "" { target *-*-* } }
}

fn labeled_match() {
    match false 'b: { // { dg-error "" "" { target *-*-* } }
        _ => {}
    }
}

fn main() {
    labeled_unsafe();
    labeled_if();
    labeled_else();
    labeled_match();
}

