struct ABig👩‍👩‍👧‍👧Family; // { dg-error "" "" { target *-*-* } }
struct 👀; // { dg-error "" "" { target *-*-* } }
impl 👀 {
    fn full_of_✨() -> 👀 { // { dg-error "" "" { target *-*-* } }
        👀
    }
}
fn i_like_to_😅_a_lot() -> 👀 { // { dg-error "" "" { target *-*-* } }
    👀::full_of✨() // { dg-error ".E0599." "" { target *-*-* } }
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}
fn main() {
    let _ = i_like_to_😄_a_lot() ➖ 4; // { dg-error ".E0425." "" { target *-*-* } }
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }

    let 🦀 = 1;// { dg-error "" "" { target *-*-* } }
    dbg!(🦀);
}

