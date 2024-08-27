fn whatever() -> i32 {
    opaque()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn opaque() -> impl Fn() -> i32 {
    || 0
}

fn main() {
    let _ = whatever();
}

