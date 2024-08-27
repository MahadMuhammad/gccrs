// Issue #118164: recovery path leaving unemitted error behind
fn bar() -> String {
    #[cfg(feature = )]
    [1, 2, 3].iter().map().collect::<String>() // { dg-error "" "" { target *-*-* } }
    #[attr] // { dg-error "" "" { target *-*-* } }
}
fn main() {
    let _ = bar();
}

