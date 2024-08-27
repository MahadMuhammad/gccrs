//@ run-rustfix

fn main() {
    let x = std::sync::Mutex::new(1usize);
    x.lock().unwrap() = 2;
// { dg-error ".E0070." "" { target *-*-* } .-1 }
    x.lock().unwrap() += 1;
// { dg-error ".E0368." "" { target *-*-* } .-1 }

    let mut y = x.lock().unwrap();
    y = 2;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    y += 1;
// { dg-error ".E0368." "" { target *-*-* } .-1 }
}

