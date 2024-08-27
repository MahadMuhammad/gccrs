//@ run-rustfix
fn main() {
    let mut v = Vec::new();
    v.push(0i32);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    v.push(0);
    v.push(1u32); // { dg-error ".E0308." "" { target *-*-* } }
// { dg-note ".E0308." "" { target *-*-* } .-1 }
// { dg-note ".E0308." "" { target *-*-* } .-2 }
// { dg-note ".E0308." "" { target *-*-* } .-3 }
// { help ".E0308." "" { target *-*-* } .-4 }
}

