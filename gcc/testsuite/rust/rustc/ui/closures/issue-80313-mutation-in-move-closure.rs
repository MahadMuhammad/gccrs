fn main() {
    let mut my_var = false;
    let callback = move || {
        my_var = true;
    };
    callback(); // { dg-error ".E0596." "" { target *-*-* } }
}

