fn main() {
    (|_, ()| ())([return, ()]);
// { dg-error ".E0057." "" { target *-*-* } .-1 }
}

