#![feature(decl_macro)]

mod foo {
    pub macro m() { Vec::<i32>::new(); ().clone() }
    fn f() { ::bar::m!(); }
}

#[no_implicit_prelude]
mod bar {
    pub macro m() {
        Vec::new(); // { dg-error ".E0433." "" { target *-*-* } }
        ().clone() // { dg-error ".E0599." "" { target *-*-* } }
    }
    fn f() {
        ::foo::m!();
        assert!(true);
    }
}

fn main() {}

