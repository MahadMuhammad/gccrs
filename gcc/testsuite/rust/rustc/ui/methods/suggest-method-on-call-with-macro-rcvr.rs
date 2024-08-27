// issue: 114131

fn main() {
    let hello = len(vec![]);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
}

