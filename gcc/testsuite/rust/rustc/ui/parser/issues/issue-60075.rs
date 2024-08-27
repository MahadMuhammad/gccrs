fn main() {}

trait T {
    fn qux() -> Option<usize> {
        let _ = if true {
        }); // { dg-error "" "" { target *-*-* } }
        Some(4)
    }

