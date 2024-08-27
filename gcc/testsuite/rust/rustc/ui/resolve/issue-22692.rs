fn main() {
    let _ = String.new();
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }

    let _ = String.default;
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }

    let _ = Vec::<()>.with_capacity(1);
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
}

macro_rules! Type {
    () => {
        ::std::cell::Cell
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { dg-error ".E0423." "" { target *-*-* } .-2 }
// { dg-error ".E0423." "" { target *-*-* } .-3 }
    };
}

macro_rules! create {
    (type method) => {
        Vec.new()
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
    };
    (type field) => {
        Vec.new
// { dg-error ".E0423." "" { target *-*-* } .-1 }
// { help ".E0423." "" { target *-*-* } .-2 }
    };
    (macro method) => {
        Type!().new(0)
// { help "" "" { target *-*-* } .-1 }
    };
}

fn interaction_with_macros() {
    //
    // Verify that we do not only suggest to replace `.` with `::` if the receiver is a
    // macro call but that we also correctly suggest to surround it with angle brackets.
    //

    Type!().get();
// { help "" "" { target *-*-* } .-1 }

    Type! {}.get;
// { help "" "" { target *-*-* } .-1 }

    //
    // Ensure that the suggestion is shown for expressions inside of macro definitions.
    //

    let _ = create!(type method);
    let _ = create!(type field);
    let _ = create!(macro method);
}

