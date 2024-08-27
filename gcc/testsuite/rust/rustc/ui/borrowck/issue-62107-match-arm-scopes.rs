fn main() {
    let e: i32;
    match e {
        ref u if true => {}
// { dg-error ".E0381." "" { target *-*-* } .-1 }
        ref v if true => {
            let tx = 0;
            &tx;
        }
        _ => (),
    }
}

