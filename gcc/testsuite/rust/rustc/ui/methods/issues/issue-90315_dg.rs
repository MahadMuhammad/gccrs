#![allow(unused)]
fn main() {
    let arr = &[0, 1, 2, 3];
    for _i in 0..arr.len().rev() {
// { dg-error ".E0689." "" { target *-*-* } .-1 }
// { dg-error ".E0689." "" { target *-*-* } .-2 }
        // The above error used to say “the method `rev` exists for type `usize`”.
        // This regression test ensures it doesn't say that any more.
    }

    // Test for #102396
    for i in 1..11.rev() {
// { dg-error ".E0689." "" { target *-*-* } .-1 }
// { help ".E0689." "" { target *-*-* } .-2 }
    }

    let end: usize = 10;
    for i in 1..end.rev() {
// { dg-error ".E0689." "" { target *-*-* } .-1 }
// { help ".E0689." "" { target *-*-* } .-2 }
    }

    for i in 1..(end + 1).rev() {
// { dg-error ".E0689." "" { target *-*-* } .-1 }
// { help ".E0689." "" { target *-*-* } .-2 }
    }

    if 1..(end + 1).is_empty() {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
    }

    if 1..(end + 1).is_sorted() {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }
    }

    let _res: i32 = 3..6.take(2).sum();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }

    let _sum: i32 = 3..6.sum();
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
// { help ".E0308." "" { target *-*-* } .-3 }

    let a = 1 as usize;
    let b = 10 as usize;

    for _a in a..=b.rev() {
// { dg-error ".E0689." "" { target *-*-* } .-1 }
// { help ".E0689." "" { target *-*-* } .-2 }
    }

    let _res = ..10.contains(3);
// { dg-error ".E0689." "" { target *-*-* } .-1 }
// { help ".E0689." "" { target *-*-* } .-2 }

    if 1..end.error_method() {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { dg-error ".E0308." "" { target *-*-* } .-2 }
        // Won't suggest
    }

    let _res = b.take(1)..a;
// { dg-error ".E0599." "" { target *-*-* } .-1 }

    let _res: i32 = ..6.take(2).sum();
// { dg-error ".E0689." "" { target *-*-* } .-1 }
// { help ".E0689." "" { target *-*-* } .-2 }
    // Won't suggest because `RangeTo` dest not implemented `take`
}

