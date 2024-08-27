// The `?` operator is still not const-evaluatable because it calls `From::from` on the error
// variant.

const fn opt() -> Option<i32> {
    let x = Some(2);
    x?; // { dg-error ".E0015." "" { target *-*-* } }
// { dg-error ".E0015." "" { target *-*-* } .-1 }
// { dg-error ".E0015." "" { target *-*-* } .-2 }
    None
}

fn main() {}

