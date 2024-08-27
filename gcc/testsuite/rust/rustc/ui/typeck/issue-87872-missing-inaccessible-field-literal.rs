pub mod foo {
    pub struct Foo {
        pub you_can_use_this_field: bool,
        you_cant_use_this_field: bool,
    }
}

fn main() {
    foo::Foo {};
// { dg-error "" "" { target *-*-* } .-1 }
}

