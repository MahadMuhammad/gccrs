// From Issue #778

enum Clam<T> { A(T) }
fn main() {
    let c;
    c = Clam::A(c);
// { dg-error ".E0275." "" { target *-*-* } .-1 }
    match c {
        Clam::A::<isize>(_) => { }
    }
}

