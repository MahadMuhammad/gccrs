struct S(i32, f32);
enum E {
    S(i32, f32),
}
struct Point4(i32, i32, i32, i32);

fn main() {
    match S(0, 1.0) {
        S(x) => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { help ".E0023." "" { target *-*-* } .-2 }
    }
    match S(0, 1.0) {
        S(_) => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { help ".E0023." "" { target *-*-* } .-2 }
// { help ".E0023." "" { target *-*-* } .-3 }
    }
    match S(0, 1.0) {
        S() => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { help ".E0023." "" { target *-*-* } .-2 }
// { help ".E0023." "" { target *-*-* } .-3 }

        // Test non-standard formatting
        S () => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { help ".E0023." "" { target *-*-* } .-2 }
// { help ".E0023." "" { target *-*-* } .-3 }
    }

    match E::S(0, 1.0) {
        E::S(x) => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { help ".E0023." "" { target *-*-* } .-2 }
    }
    match E::S(0, 1.0) {
        E::S(_) => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { help ".E0023." "" { target *-*-* } .-2 }
// { help ".E0023." "" { target *-*-* } .-3 }
    }
    match E::S(0, 1.0) {
        E::S() => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { help ".E0023." "" { target *-*-* } .-2 }
// { help ".E0023." "" { target *-*-* } .-3 }

        // Test non-standard formatting
        E::S () => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { help ".E0023." "" { target *-*-* } .-2 }
// { help ".E0023." "" { target *-*-* } .-3 }
    }
    match E::S(0, 1.0) {
        E::S => {}
// { dg-error ".E0532." "" { target *-*-* } .-1 }
// { help ".E0532." "" { target *-*-* } .-2 }
    }

    match Point4(0, 1, 2, 3) {
        Point4(   a   ,     _    ) => {}
// { dg-error ".E0023." "" { target *-*-* } .-1 }
// { help ".E0023." "" { target *-*-* } .-2 }
// { help ".E0023." "" { target *-*-* } .-3 }
    }
}

