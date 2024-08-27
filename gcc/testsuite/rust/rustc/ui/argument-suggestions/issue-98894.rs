fn main() {
    (|_, ()| ())(if true {} else {return;});
// { dg-error ".E0057." "" { target *-*-* } .-1 }
}

