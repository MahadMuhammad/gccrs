struct Struct<T>(T);
impl Struct<T>
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-note ".E0412." "" { target *-*-* } .-2 }
// { help ".E0412." "" { target *-*-* } .-3 }
where
    T: Copy,
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-note ".E0412." "" { target *-*-* } .-2 }
{
    fn method(v: Vec<u8>) { v.len(); }
}

fn main() {}

