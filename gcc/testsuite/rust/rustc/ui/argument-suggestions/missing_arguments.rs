fn one_arg(_a: i32) {}
fn two_same(_a: i32, _b: i32) {}
fn two_diff(_a: i32, _b: f32) {}
fn three_same(_a: i32, _b: i32, _c: i32) {}
fn three_diff(_a: i32, _b: f32, _c: &str) {}
fn four_repeated(_a: i32, _b: f32, _c: f32, _d: &str) {}
fn complex(_a: i32, _b: f32, _c: i32, _d: f32, _e: &str) {}

fn main() {
  one_arg(); // { dg-error ".E0061." "" { target *-*-* } }
  // The headers here show the types expected,
  // with formatting to emphasize which arguments are missing
  /*         i32     f32    */
  two_same(               ); // { dg-error ".E0061." "" { target *-*-* } }
  two_same(   1           ); // { dg-error ".E0061." "" { target *-*-* } }
  two_diff(               ); // { dg-error ".E0061." "" { target *-*-* } }
  two_diff(   1           ); // { dg-error ".E0061." "" { target *-*-* } }
  two_diff(          1.0  ); // { dg-error ".E0061." "" { target *-*-* } }

  /*           i32     i32     i32    */
  three_same(                       ); // { dg-error ".E0061." "" { target *-*-* } }
  three_same(   1                   ); // { dg-error ".E0061." "" { target *-*-* } }
  three_same(   1,      1           ); // { dg-error ".E0061." "" { target *-*-* } }

  /*           i32     f32     &str   */
  three_diff(          1.0,     ""  ); // { dg-error ".E0061." "" { target *-*-* } }
  three_diff(   1,              ""  ); // { dg-error ".E0061." "" { target *-*-* } }
  three_diff(   1,     1.0          ); // { dg-error ".E0061." "" { target *-*-* } }
  three_diff(                   ""  ); // { dg-error ".E0061." "" { target *-*-* } }
  three_diff(          1.0          ); // { dg-error ".E0061." "" { target *-*-* } }
  three_diff(   1                   ); // { dg-error ".E0061." "" { target *-*-* } }

  /*              i32     f32     f32     &str   */
  four_repeated(                               ); // { dg-error ".E0061." "" { target *-*-* } }
  four_repeated(   1,                     ""   ); // { dg-error ".E0061." "" { target *-*-* } }

  /*        i32   f32   i32   f32   &str   */
  complex(                               ); // { dg-error ".E0061." "" { target *-*-* } }
  complex(   1,                     ""   ); // { dg-error ".E0061." "" { target *-*-* } }
}

