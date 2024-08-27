fn main() {
    let x: Box<isize> = Box::new(0);

    println!("{}", x + 1);
// { dg-error ".E0369." "" { target *-*-* } .-1 }
}

