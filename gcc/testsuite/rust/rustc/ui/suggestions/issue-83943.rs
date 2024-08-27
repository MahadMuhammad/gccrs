//@ run-rustfix

fn main() {
    if true {
        "A".to_string()
    } else {
        "B" // { dg-error ".E0308." "" { target *-*-* } }
    };
}

