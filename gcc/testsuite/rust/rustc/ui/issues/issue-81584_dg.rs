//@ run-rustfix
fn main() {
        let _ = vec![vec![0, 1], vec![2]]
            .into_iter()
            .map(|y| y.iter().map(|x| x + 1))
// { dg-error ".E0515." "" { target *-*-* } .-1 }
            .collect::<Vec<_>>();
}

