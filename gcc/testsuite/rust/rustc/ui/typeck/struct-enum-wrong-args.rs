// Regression test of #86481.
struct Wrapper(i32);
struct DoubleWrapper(i32, i32);

fn main() {
    let _ = Some(3, 2); // { dg-error ".E0061." "" { target *-*-* } }
    let _ = Ok(3, 6, 2); // { dg-error ".E0061." "" { target *-*-* } }
    let _ = Ok(); // { dg-error ".E0061." "" { target *-*-* } }
    let _ = Wrapper(); // { dg-error ".E0061." "" { target *-*-* } }
    let _ = Wrapper(5, 2); // { dg-error ".E0061." "" { target *-*-* } }
    let _ = DoubleWrapper(); // { dg-error ".E0061." "" { target *-*-* } }
    let _ = DoubleWrapper(5); // { dg-error ".E0061." "" { target *-*-* } }
    let _ = DoubleWrapper(5, 2, 7); // { dg-error ".E0061." "" { target *-*-* } }
}

