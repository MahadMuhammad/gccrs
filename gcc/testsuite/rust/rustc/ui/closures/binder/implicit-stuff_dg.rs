#![feature(closure_lifetime_binder)]

fn main() {
    // Implicit types
    let _ = for<> || {};                                      // { dg-error "" "" { target *-*-* } }
    let _ = for<'a> || -> &'a _ { &() };                      // { dg-error "" "" { target *-*-* } }
    let _ = for<'a> |x| -> &'a () { x };                      // { dg-error "" "" { target *-*-* } }
    let _ = for<'a> |x: &'a _| -> &'a () { x };               // { dg-error "" "" { target *-*-* } }
    let _ = for<'a> |x: &'a Vec::<_>| -> &'a Vec::<()> { x }; // { dg-error "" "" { target *-*-* } }
    let _ = for<'a> |x: &'a Vec<()>| -> &'a Vec<_> { x };     // { dg-error "" "" { target *-*-* } }
    let _ = for<'a> |x: &'a _| -> &'a &'a () { x };           // { dg-error "" "" { target *-*-* } }
    let _ = for<'a> |x: &'a _, y, z: _| -> &'a _ {            // { dg-error "" "" { target *-*-* } }
        let _: &u8 = x;
        let _: u32 = y;
        let _: i32 = z;
        x
    };

    // Lifetime elision
    let _ = for<> |_: &()| -> () {};           // { dg-error ".E0637." "" { target *-*-* } }
    let _ = for<> |x: &()| -> &() { x };       // { dg-error ".E0637." "" { target *-*-* } }
// { dg-error ".E0637." "" { target *-*-* } .-2 }
    let _ = for<> |x: &'_ ()| -> &'_ () { x }; // { dg-error ".E0637." "" { target *-*-* } }
// { dg-error ".E0637." "" { target *-*-* } .-2 }
    let _ = for<'a> |x: &()| -> &'a () { x };  // { dg-error ".E0637." "" { target *-*-* } }
    let _ = for<'a> |x: &'a ()| -> &() { x };  // { dg-error ".E0637." "" { target *-*-* } }
// { dg-error ".E0637." "" { target *-*-* } .-1 }
}

