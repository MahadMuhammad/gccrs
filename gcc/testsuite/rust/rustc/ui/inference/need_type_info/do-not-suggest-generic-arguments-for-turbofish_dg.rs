enum OhNo<T, U> {
    A(T),
    B(U),
    C,
}

fn uwu() {
    OhNo::C::<u32, _>; // { dg-error ".E0282." "" { target *-*-* } }
}

fn main() {}

