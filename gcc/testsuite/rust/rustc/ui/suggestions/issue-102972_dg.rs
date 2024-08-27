//@ run-rustfix

fn test1() {
    let mut chars = "Hello".chars();
    for _c in chars.by_ref() {
        chars.next(); // { dg-error ".E0499." "" { target *-*-* } }
    }
}

fn test2() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    for _i in iter {
        iter.next(); // { dg-error ".E0382." "" { target *-*-* } }
    }
}

fn test3() {
    let v = vec![(), (), ()];
    let mut i = v.iter();
    for () in i.by_ref() {
        i.next(); // { dg-error ".E0499." "" { target *-*-* } }
    }
}

fn test4() {
    let v = vec![(), (), ()];
    let mut iter = v.iter();
    for () in iter {
        iter.next(); // { dg-error ".E0382." "" { target *-*-* } }
    }
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}

