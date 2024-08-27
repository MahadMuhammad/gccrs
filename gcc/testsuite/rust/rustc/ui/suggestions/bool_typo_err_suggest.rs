// Suggest the boolean value instead of emit a generic error that the value
// True is not in the scope.

fn main() {
    let x = True;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }

    let y = False;
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { help ".E0425." "" { target *-*-* } .-2 }
}

