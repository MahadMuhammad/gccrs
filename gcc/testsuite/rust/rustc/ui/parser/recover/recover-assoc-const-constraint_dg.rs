#[cfg(FALSE)]
fn syntax() {
    bar::<Item = 42>();
// { dg-error ".E0658." "" { target *-*-* } .-1 }
    bar::<Item = { 42 }>();
// { dg-error ".E0658." "" { target *-*-* } .-1 }
}

fn main() {}

