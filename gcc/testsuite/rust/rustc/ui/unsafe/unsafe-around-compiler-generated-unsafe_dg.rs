// { dg-additional-options "-frust-edition=2018" }

#![deny(unused_unsafe)]

fn main() {
    let _ = async {
        unsafe { async {}.await; } // { dg-error "" "" { target *-*-* } }
    };

    // `format_args!` expands with a compiler-generated unsafe block
    unsafe { println!("foo"); } // { dg-error "" "" { target *-*-* } }
}

