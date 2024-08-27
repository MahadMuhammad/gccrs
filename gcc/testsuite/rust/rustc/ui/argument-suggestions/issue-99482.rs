fn main() {
    let f = |_: (), f: fn()| f;
    let _f = f(main);
// { dg-error ".E0057." "" { target *-*-* } .-1 }
}

