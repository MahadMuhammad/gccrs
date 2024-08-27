fn main() {
    let mut vec = vec![0u32; 420];
    vec[vec.len() - 1] = 123; // { dg-error ".E0502." "" { target *-*-* } }
}

