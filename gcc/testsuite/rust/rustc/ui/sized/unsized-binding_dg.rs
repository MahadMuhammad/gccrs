fn main() {
    let x = *""; // { dg-error ".E0277." "" { target *-*-* } }
    println!("{}", x);
    println!("{}", x);
}

