// Regression test for #89388.

fn main() {
    let option: Option<&[u8]> = Some(b"...");
    let _ = option.map([_]::to_vec);
// { dg-error "" "" { target *-*-* } .-1 }
}

