fn thing(x: impl FnOnce(&u32, &u32, u32)) {}

fn main() {
    let f = | _ , y: &u32 , z | ();
    thing(f);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
    let f = | x, y: _  , z: u32 | ();
    thing(f);
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
// { dg-error "" "" { target *-*-* } .-3 }
// { dg-error "" "" { target *-*-* } .-4 }
}

