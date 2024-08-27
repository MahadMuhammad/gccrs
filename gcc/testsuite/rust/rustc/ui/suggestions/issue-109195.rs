fn main() {
    String::from::utf8;
// { dg-error ".E0223." "" { target *-*-* } .-1 }
// { help ".E0223." "" { target *-*-* } .-2 }
    String::from::utf8();
// { dg-error ".E0223." "" { target *-*-* } .-1 }
// { help ".E0223." "" { target *-*-* } .-2 }
    String::from::utf16();
// { dg-error ".E0223." "" { target *-*-* } .-1 }
// { help ".E0223." "" { target *-*-* } .-2 }
    String::from::method_that_doesnt_exist();
// { dg-error ".E0223." "" { target *-*-* } .-1 }
// { help ".E0223." "" { target *-*-* } .-2 }
    str::from::utf8();
// { dg-error ".E0223." "" { target *-*-* } .-1 }
// { help ".E0223." "" { target *-*-* } .-2 }
    str::from::utf8_mut();
// { dg-error ".E0223." "" { target *-*-* } .-1 }
// { help ".E0223." "" { target *-*-* } .-2 }
}

