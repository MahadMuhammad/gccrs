fn main() {
    let mut my_var = false;
    let callback = || {
        &mut my_var;
    };
    callback(); // { dg-error ".E0596." "" { target *-*-* } }
}

