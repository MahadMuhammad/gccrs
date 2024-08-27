fn main() {
    #[derive(Copy, Clone)]
    union U8AsBool {
        n: u8,
        b: bool,
    }

    let x = U8AsBool { n: 1 };
    unsafe {
        match x {
            // exhaustive
            U8AsBool { n: 2 } => {}
            U8AsBool { b: true } => {}
            U8AsBool { b: false } => {}
        }
        match x {
            // exhaustive
            U8AsBool { b: true } => {}
            U8AsBool { n: 0 } => {}
            U8AsBool { n: 1.. } => {}
        }
        match x {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
            U8AsBool { b: true } => {}
            U8AsBool { n: 1.. } => {}
        }
        // Our approach can report duplicate witnesses sometimes.
        match (x, true) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
            (U8AsBool { b: true }, true) => {}
            (U8AsBool { b: false }, true) => {}
            (U8AsBool { n: 1.. }, true) => {}
        }
    }
}

