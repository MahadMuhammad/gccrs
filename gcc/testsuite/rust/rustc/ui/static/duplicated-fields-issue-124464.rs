// Don't const eval fields with ambiguous layout.
// See issues #125842 and #124464.

enum TestOption<T> {
    TestSome(T),
    TestSome(T),
// { dg-error ".E0428." "" { target *-*-* } .-1 }
}

pub struct Request {
    bar: TestOption<u64>,
    bar: u8,
// { dg-error ".E0124." "" { target *-*-* } .-1 }
}

fn default_instance() -> &'static Request {
    static instance: Request = Request { bar: 17 };
    &instance
}

pub fn main() {}

