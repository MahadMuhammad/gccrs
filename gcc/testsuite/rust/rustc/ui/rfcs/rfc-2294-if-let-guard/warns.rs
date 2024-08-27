#![feature(if_let_guard)]

#[deny(irrefutable_let_patterns)]
fn irrefutable_let_guard() {
    match Some(()) {
        Some(x) if let () = x => {}
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }
}

#[deny(unreachable_patterns)]
fn unreachable_pattern() {
    match Some(()) {
        x if let None | None = x => {}
// { dg-error "" "" { target *-*-* } .-1 }
        _ => {}
    }
}

fn main() {}

