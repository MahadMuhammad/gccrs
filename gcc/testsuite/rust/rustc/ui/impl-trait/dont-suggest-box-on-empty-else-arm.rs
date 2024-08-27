fn test() -> impl std::fmt::Debug {
    if true {
        "boo2"
    } else {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }
}

fn main() {}

