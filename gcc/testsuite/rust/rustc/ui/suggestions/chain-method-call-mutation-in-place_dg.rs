fn main() {
    let x: Vec<i32> = vec![1, 2, 3].into_iter().collect::<Vec<i32>>().sort_by_key(|i| i); // { dg-error ".E0308." "" { target *-*-* } }
    vec![1, 2, 3].into_iter().collect::<Vec<i32>>().sort_by_key(|i| i).sort(); // { dg-error ".E0599." "" { target *-*-* } }
}

fn foo(mut s: String) -> String {
    s.push_str("asdf") // { dg-error ".E0308." "" { target *-*-* } }
}

