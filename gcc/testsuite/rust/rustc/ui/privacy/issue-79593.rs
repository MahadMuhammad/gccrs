mod foo {
    pub struct Pub { private: () }

    pub enum Enum {
        Variant { x: (), y: () },
        Other
    }

    fn correct() {
        Pub {};
// { dg-error ".E0063." "" { target *-*-* } .-1 }
        Enum::Variant { x: () };
// { dg-error ".E0063." "" { target *-*-* } .-1 }
    }
}

fn correct() {
    foo::Pub {};
// { dg-error "" "" { target *-*-* } .-1 }
}

fn wrong() {
    foo::Enum::Variant { x: () };
// { dg-error ".E0063." "" { target *-*-* } .-1 }
    foo::Enum::Variant { };
// { dg-error ".E0063." "" { target *-*-* } .-1 }
}

fn main() {}

