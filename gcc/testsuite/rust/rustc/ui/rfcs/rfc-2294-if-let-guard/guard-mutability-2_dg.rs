// Check mutable reference bindings cannot be mutated by an if-let guard.

#![feature(if_let_guard)]

fn main() {
    let mut x: Option<Option<i32>> = Some(Some(6));
    match x {
        Some(ref mut y) if let Some(ref mut z) = *y => {
// { dg-error ".E0596." "" { target *-*-* } .-1 }
            let _: &mut i32 = z;
        }
        _ => {}
    }
}

