fn main() {
    let elem = 6i8;
    let e2 = 230;
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }

    let mut vec = Vec::new();

    vec.push(e2);
    vec.push(elem);

    println!("{:?}", vec);
}

