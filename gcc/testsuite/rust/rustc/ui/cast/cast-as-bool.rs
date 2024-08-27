fn main() {
    let u = 5 as bool; // { dg-error ".E0054." "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
// { suggestion "" "" { target *-*-* } .-3 }

    let t = (1 + 2) as bool; // { dg-error ".E0054." "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }
// { suggestion "" "" { target *-*-* } .-3 }

    let _ = 5_u32 as bool; // { dg-error ".E0054." "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }

    let _ = 64.0_f64 as bool; // { dg-error ".E0054." "" { target *-*-* } }
// { help "" "" { target *-*-* } .-2 }

    // Enums that can normally be cast to integers can't be cast to `bool`, just like integers.
    // Note that enums that cannot be cast to integers can't be cast to anything at *all*
    // so that's not tested here.
    enum IntEnum {
        Zero,
        One,
        Two
    }
    let _ = IntEnum::One as bool; // { dg-error ".E0054." "" { target *-*-* } }

    fn uwu(_: u8) -> String {
        todo!()
    }

    unsafe fn owo() {}

    // fn item to bool
    let _ = uwu as bool; // { dg-error ".E0054." "" { target *-*-* } }
    // unsafe fn item
    let _ = owo as bool; // { dg-error ".E0054." "" { target *-*-* } }

    // fn ptr to bool
    let _ = uwu as fn(u8) -> String as bool; // { dg-error ".E0054." "" { target *-*-* } }

    let _ = 'x' as bool; // { dg-error ".E0054." "" { target *-*-* } }

    let ptr = 1 as *const ();

    let _ = ptr as bool; // { dg-error ".E0054." "" { target *-*-* } }

    let v = "hello" as bool;
// { dg-error ".E0606." "" { target *-*-* } .-1 }
// { help ".E0606." "" { target *-*-* } .-2 }
}

