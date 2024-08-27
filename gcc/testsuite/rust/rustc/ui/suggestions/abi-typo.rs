//@ run-rustfix
extern "cdedl" fn cdedl() {} // { dg-error ".E0703." "" { target *-*-* } }

fn main() {
    cdedl();
}

