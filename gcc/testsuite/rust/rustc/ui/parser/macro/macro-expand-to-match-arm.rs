macro_rules! arm {
    ($pattern:pat => $block:block) => {
        $pattern => $block
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
    };
}

fn main() {
    let x = Some(1);
    match x {
        Some(1) => {},
        arm!(None => {}),
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        Some(2) => {},
        _ => {},
    };
}

