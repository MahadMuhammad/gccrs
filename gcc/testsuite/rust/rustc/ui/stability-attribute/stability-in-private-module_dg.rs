fn main() {
    let _ = std::sys::os::errno();
// { dg-error ".E0603." "" { target *-*-* } .-1 }
}

