trait Foo<T> {}

impl Foo<T: Default> for String {}
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { help ".E0229." "" { target *-*-* } .-2 }

impl Foo<T: 'a + Default> for u8 {}
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { help ".E0229." "" { target *-*-* } .-2 }
// { dg-error ".E0229." "" { target *-*-* } .-3 }
// { help ".E0229." "" { target *-*-* } .-4 }

impl<T> Foo<T: Default> for u16 {}
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { help ".E0229." "" { target *-*-* } .-2 }

impl<'a> Foo<T: 'a + Default> for u32 {}
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { help ".E0229." "" { target *-*-* } .-2 }

trait Bar<T, K> {}

impl Bar<T: Default, K: Default> for String {}
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { help ".E0229." "" { target *-*-* } .-2 }

impl<T> Bar<T, K: Default> for u8 {}
// { dg-error ".E0229." "" { target *-*-* } .-1 }
// { help ".E0229." "" { target *-*-* } .-2 }
// { dg-error ".E0229." "" { target *-*-* } .-3 }
// { help ".E0229." "" { target *-*-* } .-4 }

fn main() {}

