fn main() {
    println!("Custom backtrace: {}", std::backtrace::Backtrace::forced_capture());
// { dg-error ".E0599." "" { target *-*-* } .-1 }
}

