//@ run-rustfix
mod foo {
    pub mod bar {
        pub mod baz {
            pub use std::vec::Vec as MyVec;
        }
    }
}

mod u {
    fn _a() {
        let _: Vec<i32> = super::foo::baf::baz::MyVec::new(); // { dg-error ".E0433." "" { target *-*-* } }
    }
}

mod v {
    fn _b() {
        let _: Vec<i32> = fox::bar::baz::MyVec::new(); // { dg-error ".E0433." "" { target *-*-* } }
    }
}

fn main() {
    let _t: Vec<i32> = vec::new(); // { dg-error ".E0433." "" { target *-*-* } }
    type _B = vec::Vec::<u8>; // { dg-error ".E0433." "" { target *-*-* } }
    let _t = std::sync_error::atomic::AtomicBool::new(true); // { dg-error ".E0433." "" { target *-*-* } }
}

