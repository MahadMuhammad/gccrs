fn process_slice(data: &[i32]) {
    todo!()
}

fn main() {
    let some_generated_vec = (0..10).collect();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-error ".E0277." "" { target *-*-* } .-2 }
// { dg-error ".E0277." "" { target *-*-* } .-3 }
// { dg-note ".E0277." "" { target *-*-* } .-4 }
// { dg-note ".E0277." "" { target *-*-* } .-5 }
// { dg-note ".E0277." "" { target *-*-* } .-6 }
// { dg-note ".E0277." "" { target *-*-* } .-7 }
// { dg-note ".E0277." "" { target *-*-* } .-8 }
// { dg-note ".E0277." "" { target *-*-* } .-9 }
    process_slice(&some_generated_vec);

    let some_generated_vec = (0..10).collect();
// { dg-error ".E0277." "" { target *-*-* } .-1 }
// { dg-note ".E0277." "" { target *-*-* } .-2 }
// { dg-note ".E0277." "" { target *-*-* } .-3 }
    process_slice(some_generated_vec);
}

