fn main() {
    let a = "a";
    let b = "b";

    println!("{a} {b} {} {} {c} {}", c = "c");
// { dg-error "" "" { target *-*-* } .-1 }

    let n = 1;
    println!("{a:.n$} {b:.*}");
// { dg-error "" "" { target *-*-* } .-1 }
}

