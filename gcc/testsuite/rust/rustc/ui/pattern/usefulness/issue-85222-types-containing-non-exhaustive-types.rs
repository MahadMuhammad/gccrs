struct A<T> {
    a: T,
}

struct B<T, U>(T, U);

fn main() {
    match 0 {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        0 => (),
        1..=usize::MAX => (),
    }

    match (0usize, 0usize) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        (0, 0) => (),
        (1..=usize::MAX, 1..=usize::MAX) => (),
    }

    match (0isize, 0usize) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        (isize::MIN..=isize::MAX, 0) => (),
        (isize::MIN..=isize::MAX, 1..=usize::MAX) => (),
    }

    // Should not report note about usize not having fixed max value
    match Some(1usize) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        None => {}
    }

    match Some(4) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        Some(0) => (),
        Some(1..=usize::MAX) => (),
        None => (),
    }

    match Some(Some(Some(0))) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        Some(Some(Some(0))) => (),
        Some(Some(Some(1..=usize::MAX))) => (),
        Some(Some(None)) => (),
        Some(None) => (),
        None => (),
    }

    match (A { a: 0usize }) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        A { a: 0 } => (),
        A { a: 1..=usize::MAX } => (),
    }

    match B(0isize, 0usize) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        B(isize::MIN..=isize::MAX, 0) => (),
        B(isize::MIN..=isize::MAX, 1..=usize::MAX) => (),
    }

    // Should report only the note about usize not having fixed max value and not report
    // report the note about isize
    match B(0isize, 0usize) {
// { dg-error ".E0004." "" { target *-*-* } .-1 }
        B(_, 0) => (),
        B(_, 1..=usize::MAX) => (),
    }
}

