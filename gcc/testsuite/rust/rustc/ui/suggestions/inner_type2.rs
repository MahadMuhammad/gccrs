pub struct Struct<T> {
    pub p: T,
}

impl<T> Struct<T> {
    pub fn method(&self) {}

    pub fn some_mutable_method(&mut self) {}
}

thread_local! {
    static STRUCT: Struct<u32> = Struct {
        p: 42_u32
    };
}

fn main() {
    STRUCT.method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }

    let item = std::mem::MaybeUninit::new(Struct { p: 42_u32 });
    item.method();
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { help ".E0599." "" { target *-*-* } .-2 }
}

