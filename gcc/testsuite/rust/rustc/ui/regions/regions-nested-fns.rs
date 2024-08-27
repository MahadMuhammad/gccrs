fn ignore<T>(t: T) {}

fn nested<'x>(x: &'x isize) {
    let y = 3;
    let mut ay = &y;
// { dg-error ".E0597." "" { target *-*-* } .-1 }

    ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
        ay = x;
        ay = &y;
// { dg-error ".E0597." "" { target *-*-* } .-1 }
        ay = z;
// { dg-error ".E0521." "" { target *-*-* } .-1 }
    }));

    ignore::< Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
        if false { return x; }
// { dg-error "" "" { target *-*-* } .-1 }
        if false { return ay; }
        return z;
    }));
}

fn main() {}

