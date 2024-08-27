#![deny(dead_code)]

enum Category {
    Dead, // { dg-error "" "" { target *-*-* } }
    Used,
}

trait UnusedTrait { // { dg-error "" "" { target *-*-* } }
    fn this_is_unused(&self) -> Category {
        Category::Dead
    }
}

struct UnusedStruct; // { dg-error "" "" { target *-*-* } }

impl UnusedTrait for UnusedStruct {
    fn this_is_unused(&self) -> Category {
        Category::Used
    }
}

mod private {
    #[derive(Debug)]
    struct UnusedStruct; // { dg-error "" "" { target *-*-* } }
}

fn main() {
    let _c = Category::Used;
}

