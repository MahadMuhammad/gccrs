// Suggest print macro when user erroneously uses printf

fn main() {
    let x = 4;
    printf("%d", x);
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }
}

