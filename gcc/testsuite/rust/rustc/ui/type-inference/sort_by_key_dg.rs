fn main() {
    let mut lst: [([i32; 10], bool); 10] = [([0; 10], false); 10];
    lst.sort_by_key(|&(v, _)| v.iter().sum()); // { dg-error ".E0283." "" { target *-*-* } }
    println!("{:?}", lst);
}

