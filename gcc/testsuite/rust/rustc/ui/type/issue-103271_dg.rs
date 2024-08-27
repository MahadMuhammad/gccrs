fn main() {
    let iter_fun = <&[u32]>::iter;
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-error ".E0599." "" { target *-*-* } .-2 }
// { help ".E0599." "" { target *-*-* } .-3 }
    for item in iter_fun(&[1,1]) {
        let x: &u32 = item;
        assert_eq!(x, &1);
    }
    let iter_fun2 = <(&[u32])>::iter;
// { dg-error ".E0599." "" { target *-*-* } .-1 }
// { dg-error ".E0599." "" { target *-*-* } .-2 }
// { help ".E0599." "" { target *-*-* } .-3 }
    for item2 in iter_fun2(&[1,1]) {
        let x: &u32 = item2;
        assert_eq!(x, &1);
    }
}

