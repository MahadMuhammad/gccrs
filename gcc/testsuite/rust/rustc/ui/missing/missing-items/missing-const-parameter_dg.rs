struct Struct<const N: usize>;

impl Struct<{ N }> {}
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }

fn func0(_: Struct<{ N }>) {}
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }

fn func1(_: [u8; N]) {}
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }

fn func2<T>(_: [T; N]) {}
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }

struct Image<const R: usize>([[u32; C]; R]);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }
// { help ".E0425." "" { target *-*-* } .-3 }

fn main() {}

