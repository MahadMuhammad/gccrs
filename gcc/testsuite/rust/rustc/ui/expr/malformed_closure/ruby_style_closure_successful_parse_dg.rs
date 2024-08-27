const x: usize =42;
fn main() {
    let p = Some(45).and_then({|x| // { dg-error ".E0277." "" { target *-*-* } }
        1 + 1;
        Some(x * 2)
    });
}

