fn main() {
    let slice = [1,2,3,4];
    let vec = vec![1,2,3,4];

    slice.count(); // { dg-error ".E0599." "" { target *-*-* } }
    vec.count(); // { dg-error ".E0599." "" { target *-*-* } }
    vec.as_slice().count(); // { dg-error ".E0599." "" { target *-*-* } }
}

