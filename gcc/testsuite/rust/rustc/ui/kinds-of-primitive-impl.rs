impl u8 {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    pub const B: u8 = 0;
}

impl str {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    fn foo() {}
    fn bar(self) {} // { dg-error ".E0277." "" { target *-*-* } }
}

impl char {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    pub const B: u8 = 0;
    pub const C: u8 = 0;
    fn foo() {}
    fn bar(self) {}
}

struct MyType;
impl &MyType {
// { dg-error ".E0390." "" { target *-*-* } .-1 }
    pub fn for_ref(self) {}
}

fn main() {}

