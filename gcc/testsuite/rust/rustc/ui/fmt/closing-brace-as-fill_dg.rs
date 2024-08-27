// issue: 112732

// `}` is typoed since it is interpreted as a fill character rather than a closing bracket

fn main() {
    println!("Hello, world! {0:}<3", 2);
// { dg-error "" "" { target *-*-* } .-1 }
}

