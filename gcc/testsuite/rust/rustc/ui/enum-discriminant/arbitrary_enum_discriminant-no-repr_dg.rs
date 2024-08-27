#![crate_type="lib"]

enum Enum {
// { dg-error ".E0732." "" { target *-*-* } .-1 }
  Unit = 1,
  Tuple() = 2,
  Struct{} = 3,
}

