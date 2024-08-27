// We used to fatal error without any useful diagnostic when we had an unexpected
// token due to a strange interaction between the sequence parsing code and the
// param/lifetime parsing code.

fn hello() -> impl Sized + use<'a {}> {}
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {}

