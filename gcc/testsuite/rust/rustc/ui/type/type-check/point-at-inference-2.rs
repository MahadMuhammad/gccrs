fn bar(_: Vec<i32>) {}
fn baz(_: &Vec<&i32>) {}
fn main() {
    let v = vec![&1];
    bar(v); // { dg-error ".E0308." "" { target *-*-* } }
    let v = vec![];
    baz(&v);
    baz(&v);
    bar(v); // { dg-error ".E0308." "" { target *-*-* } }
    let v = vec![];
    baz(&v);
    bar(v); // { dg-error ".E0308." "" { target *-*-* } }
}

