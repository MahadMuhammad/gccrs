fn digit() -> str {
    return {};
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}
fn main() {
    let [_y..] = [Box::new(1), Box::new(2)];
// { dg-error ".E0527." "" { target *-*-* } .-1 }
// { dg-error ".E0527." "" { target *-*-* } .-2 }
// { dg-error ".E0527." "" { target *-*-* } .-3 }
}

