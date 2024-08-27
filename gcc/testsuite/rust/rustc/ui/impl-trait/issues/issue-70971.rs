fn main() {
    let x : (impl Copy,) = (true,);
// { dg-error ".E0562." "" { target *-*-* } .-1 }
}

