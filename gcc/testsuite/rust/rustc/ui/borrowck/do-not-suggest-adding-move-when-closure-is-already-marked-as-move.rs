fn main() {
    let mut vec: Vec<i32> = Vec::new();
    let closure = move || {
        vec.clear();
        let mut iter = vec.iter();
        move || { iter.next() } // { dg-error "" "" { target *-*-* } }
    };
}

