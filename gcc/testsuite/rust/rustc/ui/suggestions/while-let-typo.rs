fn main() {
    let foo = Some(0);
    let bar = None;
    while Some(x) = foo {} // { dg-error ".E0425." "" { target *-*-* } }
    while Some(foo) = bar {} // { dg-error ".E0308." "" { target *-*-* } }
    while 3 = foo {} // { dg-error ".E0308." "" { target *-*-* } }
    while Some(3) = foo {} // { dg-error ".E0070." "" { target *-*-* } }
    while x = 5 {} // { dg-error ".E0425." "" { target *-*-* } }
}

