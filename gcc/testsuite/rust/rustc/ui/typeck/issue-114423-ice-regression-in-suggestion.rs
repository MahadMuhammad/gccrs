struct RGB {
    g: f64,
    b: f64,
}

fn main() {
    let (r, alone_in_path, b): (f32, f32, f32) = (e.clone(), e.clone());
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
    let _ = RGB { r, g, b };
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { dg-error ".E0308." "" { target *-*-* } .-3 }
}

