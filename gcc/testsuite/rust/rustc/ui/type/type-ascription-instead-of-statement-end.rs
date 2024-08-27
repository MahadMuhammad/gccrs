fn main() {
    println!("test"): // { dg-error "" "" { target *-*-* } }
    0;
}

fn foo() {
    println!("test"): 0; // { dg-error "" "" { target *-*-* } }
}

