fn main() {
    let container = vec![Some(1), Some(2), None];

    let mut i = 0;
    while if let Some(thing) = container.get(i) {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        println!("{:?}", thing);
        i += 1;
    }
}
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

