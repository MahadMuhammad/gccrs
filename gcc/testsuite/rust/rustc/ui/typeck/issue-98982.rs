fn foo() -> i32 {
    for i in 0..0 { // { dg-error ".E0308." "" { target *-*-* } }
        return i;
    } // { help "" "" { target *-*-* } }
}

fn main() {}

