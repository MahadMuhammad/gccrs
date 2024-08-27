//@ run-rustfix
fn main() {

let _x: isize<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: i8<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: i16<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: i32<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: i64<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: usize<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: u8<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: u16<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: u32<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: u64<isize>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: char<isize>; // { dg-error ".E0109." "" { target *-*-* } }

let _x: isize<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: i8<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: i16<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: i32<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: i64<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: usize<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: u8<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: u16<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: u32<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: u64<'static>; // { dg-error ".E0109." "" { target *-*-* } }
let _x: char<'static>; // { dg-error ".E0109." "" { target *-*-* } }

}

