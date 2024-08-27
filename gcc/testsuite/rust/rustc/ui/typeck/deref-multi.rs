fn a(x: &&i32) -> i32 {
    x
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn a2(x: &&&&&i32) -> i32 {
    x
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn b(x: &i32) -> i32 {
    &x
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn c(x: Box<i32>) -> i32 {
    &x
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn d(x: std::sync::Mutex<&i32>) -> i32 {
    x.lock().unwrap()
// { dg-error ".E0308." "" { target *-*-* } .-1 }
}

fn main() {}

