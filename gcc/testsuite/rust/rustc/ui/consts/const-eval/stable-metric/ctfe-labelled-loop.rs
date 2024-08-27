//@ check-fail
//@ compile-flags: -Z tiny-const-eval-limit

const fn labelled_loop(n: u32) -> u32 {
    let mut i = 0;
    'mylabel: loop {
// { dg-error "" "" { target *-*-* } .-1 }
        if i > n {
            break 'mylabel;
        }
        i += 1;
    }
    0
}

const X: u32 = labelled_loop(19);

fn main() {
    println!("{X}");
}

