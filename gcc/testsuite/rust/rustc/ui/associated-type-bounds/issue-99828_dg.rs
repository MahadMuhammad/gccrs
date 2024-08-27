fn get_iter(vec: &[i32]) -> impl Iterator<Item = {}> + '_ {
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }
    vec.iter()
}

fn main() {
    let vec = Vec::new();
    let mut iter = get_iter(&vec);
    iter.next();
}

