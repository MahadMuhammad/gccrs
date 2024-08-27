mod to_reuse {
    pub fn foo() {}
}

reuse to_reuse::foo;
// { dg-error ".E0658." "" { target *-*-* } .-1 }

fn main() {}

