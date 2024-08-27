fn main() {
    match &[1, 2, 3][..] {
        [1, rest..] => println!("{rest}"),
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-error ".E0658." "" { target *-*-* } .-2 }
// { dg-error ".E0658." "" { target *-*-* } .-3 }
        _ => {}
    }
    match &[4, 5, 6][..] {
        [] => {}
        [_, ..tail] => println!("{tail}"),
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
    }
    match &[7, 8, 9][..] {
        [] => {}
        [_, ...tail] => println!("{tail}"),
// { dg-error ".E0425." "" { target *-*-* } .-1 }
// { dg-error ".E0425." "" { target *-*-* } .-2 }
// { dg-error ".E0425." "" { target *-*-* } .-3 }
    }
}

