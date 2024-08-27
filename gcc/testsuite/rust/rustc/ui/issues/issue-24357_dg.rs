struct NoCopy; // { dg-note "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
fn main() {
   let x = NoCopy;
// { dg-note "" "" { target *-*-* } .-1 }
   let f = move || { let y = x; };
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
   let z = x;
// { dg-error ".E0382." "" { target *-*-* } .-1 }
// { dg-note ".E0382." "" { target *-*-* } .-2 }
}

