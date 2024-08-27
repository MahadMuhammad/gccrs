fn main() {
    match true {
        _ if let true = true && true => {}
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
        _ => {}
    }
}

