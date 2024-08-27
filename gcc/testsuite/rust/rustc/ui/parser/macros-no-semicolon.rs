fn main() {
    assert_eq!(1, 2) // { dg-error "" "" { target *-*-* } }
    assert_eq!(3, 4) // { dg-error "" "" { target *-*-* } }
    println!("hello");
}

