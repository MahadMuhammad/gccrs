fn main() {

    let x: Box<_> = 5.into();
    let y = x;

    println!("{}", *x); // { dg-error ".E0382." "" { target *-*-* } }
    y.clone();
}

