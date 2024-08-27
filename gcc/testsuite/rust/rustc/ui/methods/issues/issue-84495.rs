fn main() {
    let x: i32 = 1;
    println!("{:?}", x.count()); // { dg-error ".E0599." "" { target *-*-* } }
}

