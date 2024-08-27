macro_rules! statement {
    () => {;}; // { dg-error "" "" { target *-*-* } }
}

fn main() {
    let _ = statement!();
}

