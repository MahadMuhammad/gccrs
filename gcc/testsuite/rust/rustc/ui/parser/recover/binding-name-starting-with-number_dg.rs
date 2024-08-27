fn 1234test() {
// { dg-error "" "" { target *-*-* } .-1 }
    if let 123 = 123 { println!("yes"); }

    if let 2e1 = 123 {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }

    let 23name = 123;
// { dg-error "" "" { target *-*-* } .-1 }
}
fn foo() {
    let 2x: i32 = 123;
// { dg-error "" "" { target *-*-* } .-1 }
}
fn bar() {
    let 1x = 123;
// { dg-error "" "" { target *-*-* } .-1 }
}

fn main() {}

