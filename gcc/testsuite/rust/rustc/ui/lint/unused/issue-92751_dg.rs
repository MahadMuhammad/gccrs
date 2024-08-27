#[deny(unused)]
pub fn broken(x: Option<()>) -> i32 {
    match x {
        Some(()) => (1), // { dg-error "" "" { target *-*-* } }
        None => (2), // { dg-error "" "" { target *-*-* } }
    }
}

fn main() { }

