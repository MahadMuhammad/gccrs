fn main() {
    let v = Vec::new(); // { dg-error ".E0596." "" { target *-*-* } }
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
    v.push(0);
}

