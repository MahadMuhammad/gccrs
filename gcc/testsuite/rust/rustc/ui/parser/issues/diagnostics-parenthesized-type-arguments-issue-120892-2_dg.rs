fn main() {
  foo::/* definitely not harmful comment */(123, "foo") -> (u32); // { dg-error "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
}

