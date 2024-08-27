mod module {
    pub struct SomeTupleStruct(u8);
    pub struct SomeRegularStruct {
        foo: u8
    }

    impl SomeTupleStruct {
        pub fn new() -> Self {
            Self(0)
        }
    }
    impl SomeRegularStruct {
        pub fn new() -> Self {
            Self { foo: 0 }
        }
    }
}

use module::{SomeTupleStruct, SomeRegularStruct};

fn main() {
    let _ = SomeTupleStruct.new();
// { dg-error ".E0423." "" { target *-*-* } .-1 }
    let _ = SomeRegularStruct.new();
// { dg-error ".E0423." "" { target *-*-* } .-1 }
}

