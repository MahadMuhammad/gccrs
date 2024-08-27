struct A;

fn main() {
  println!("{:?}", 1.0 as *const A); // { dg-error ".E0606." "" { target *-*-* } }
}

