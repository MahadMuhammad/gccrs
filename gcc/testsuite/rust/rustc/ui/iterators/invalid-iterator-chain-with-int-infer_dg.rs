fn main() {
    let x = Some(()).iter().map(|()| 1).sum::<f32>();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
}

