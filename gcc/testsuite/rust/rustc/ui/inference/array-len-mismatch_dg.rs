fn returns_arr() -> [u8; 2] {
    [1, 2]
}

fn main() {
    let wrong: [u8; 3] = [10, 20];
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
    let wrong: [u8; 3] = returns_arr();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
}

