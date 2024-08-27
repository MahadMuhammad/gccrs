fn main() {
    let Some(x) = Some(2) else {
        panic!("{}", x); // { dg-error ".E0425." "" { target *-*-* } }
    };
}

