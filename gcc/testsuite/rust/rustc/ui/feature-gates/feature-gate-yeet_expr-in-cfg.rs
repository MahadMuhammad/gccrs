//@ compile-flags: --edition 2021

pub fn demo() -> Option<i32> {
    #[cfg(FALSE)]
    {
        do yeet // { dg-error ".E0658." "" { target *-*-* } }
    }

    Some(1)
}

#[cfg(FALSE)]
pub fn alternative() -> Result<(), String> {
    do yeet "hello"; // { dg-error ".E0658." "" { target *-*-* } }
}

fn main() {
    demo();
}

