//@ run-rustfix

fn main() {
    let mut x = vec![1usize];
    x.last_mut().unwrap() = 2;
// { dg-error ".E0070." "" { target *-*-* } .-1 }
    x.last_mut().unwrap() += 1;
// { dg-error ".E0368." "" { target *-*-* } .-1 }

    let y = x.last_mut().unwrap();
    y = 2;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    y += 1;
// { dg-error ".E0368." "" { target *-*-* } .-1 }
}

