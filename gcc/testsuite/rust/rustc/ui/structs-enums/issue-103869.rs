//@ run-rustfix

enum VecOrMap {
// { help "" "" { target *-*-* } .-1 }
    vec: Vec<usize>,
// { dg-error "" "" { target *-*-* } .-1 }
// { help "" "" { target *-*-* } .-2 }
}

fn main() {
    let o = VecOrMap { vec: vec![1, 2, 3] };
    println!("{:?}", o.vec);
}

