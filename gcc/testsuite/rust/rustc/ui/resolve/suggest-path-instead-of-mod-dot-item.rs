// Beginners write `mod.item` when they should write `mod::item`.
// This tests that we suggest the latter when we encounter the former.

pub mod a {
    pub const I: i32 = 1;

    pub fn f() -> i32 { 2 }

    pub mod b {
        pub const J: i32 = 3;

        pub fn g() -> i32 { 4 }
    }
}

fn h1() -> i32 {
    a.I
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
}

fn h2() -> i32 {
    a.g()
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
}

fn h3() -> i32 {
    a.b.J
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
}

fn h4() -> i32 {
    a::b.J
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
// { help ".E0423." "" { target *-*-* } .-3 }
}

fn h5() {
    a.b.f();
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
    let v = Vec::new();
    v.push(a::b);
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
}

fn h6() -> i32 {
    a::b.f()
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
// { help ".E0423." "" { target *-*-* } .-3 }
}

fn h7() {
    a::b
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
}

fn h8() -> i32 {
    a::b()
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
}

macro_rules! module {
    () => {
        a
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { dg-error ".E0423." "" { target *-*-* } .-2 }
    };
}

macro_rules! create {
    (method) => {
        a.f()
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
    };
    (field) => {
        a.f
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
    };
}

fn h9() {
    //
    // Note that if the receiver is a macro call, we do not want to suggest to replace
    // `.` with `::` as that would be a syntax error.
    // Since the receiver is a module and not a type, we cannot suggest to surround
    // it with angle brackets.
    //

    module!().g::<()>(); // no `help` here!

    module!().g; // no `help` here!

    //
    // Ensure that the suggestion is shown for expressions inside of macro definitions.
    //

    let _ = create!(method);
    let _ = create!(field);
}

fn main() {}

