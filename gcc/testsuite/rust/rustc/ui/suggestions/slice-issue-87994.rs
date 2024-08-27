fn main() {
  let v = vec![1i32, 2, 3];
  for _ in v[1..] {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
  }
  struct K {
    n: i32,
  }
  let mut v2 = vec![K { n: 1 }, K { n: 1 }, K { n: 1 }];
  for i2 in v2[1..] {
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
    i2.n = 2;
  }
}

