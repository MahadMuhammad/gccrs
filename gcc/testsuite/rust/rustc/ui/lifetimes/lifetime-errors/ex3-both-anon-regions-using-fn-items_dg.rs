fn foo(x:fn(&u8, &u8), y: Vec<&u8>, z: &u8) {
  y.push(z);
// { dg-error ".E0596." "" { target *-*-* } .-1 }
// { dg-error ".E0596." "" { target *-*-* } .-2 }
}

fn main() { }

