fn main() {
    let mut x = 1i32;
    let y = Box::new(|| 1);
    x = y;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

