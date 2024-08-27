// { dg-additional-options "-frust-edition=2021" }

// Test that move closures drop derefs with `capture_disjoint_fields` enabled.

#![feature(rustc_attrs)]

fn simple_move_closure() {
    struct S(String);
    struct T(S);

    let t = T(S("s".into()));
    let mut c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        t.0.0 = "new S".into();
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };
    c();
}

// Test move closure use reborrows when using references
fn simple_ref() {
    let mut s = 10;
    let ref_s = &mut s;

    let mut c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        *ref_s += 10;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };
    c();
}

// Test move closure use reborrows when using references
fn struct_contains_ref_to_another_struct_1() {
    struct S(String);
    struct T<'a>(&'a mut S);

    let mut s = S("s".into());
    let t = T(&mut s);

    let mut c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        t.0.0 = "new s".into();
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();
}

// Test that we can use reborrows to read data of Copy types
// i.e. without truncating derefs
fn struct_contains_ref_to_another_struct_2() {
    struct S(i32);
    struct T<'a>(&'a S);

    let s = S(0);
    let t = T(&s);

    let mut c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let _t = t.0.0;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();
}

// Test that we can use truncate to move out of !Copy types
fn struct_contains_ref_to_another_struct_3() {
    struct S(String);
    struct T<'a>(&'a S);

    let s = S("s".into());
    let t = T(&s);

    let mut c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let _t = t.0.0;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();
}

// Test that derefs of box are truncated in move closures
fn truncate_box_derefs() {
    struct S(i32);


    // Content within the box is moved within the closure
    let b = Box::new(S(10));
    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        let _t = b.0;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();

    // Content within the box is used by a shared ref and the box is the root variable
    let b = Box::new(S(10));

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        println!("{}", b.0);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();

    // Content within the box is used by a shared ref and the box is not the root variable
    let b = Box::new(S(10));
    let t = (0, b);

    let c = #[rustc_capture_analysis]
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
    move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-error "" "" { target *-*-* } .-2 }
        println!("{}", t.1.0);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };
}

struct Foo { x: i32 }

// Ensure that even in move closures, if the data is not owned by the root variable
// then we don't truncate the derefs or a ByValue capture, rather do a reborrow
fn box_mut_1() {
    let mut foo = Foo { x: 0 } ;

    let p_foo = &mut foo;
    let box_p_foo = Box::new(p_foo);

    let c = #[rustc_capture_analysis] move || box_p_foo.x += 10;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
// { dg-error ".E0658." "" { target *-*-* } .-4 }
// { dg-note ".E0658." "" { target *-*-* } .-5 }
// { dg-error ".E0658." "" { target *-*-* } .-6 }
// { dg-note ".E0658." "" { target *-*-* } .-7 }
}

// Ensure that even in move closures, if the data is not owned by the root variable
// then we don't truncate the derefs or a ByValue capture, rather do a reborrow
fn box_mut_2() {
    let foo = Foo { x: 0 } ;

    let mut box_foo = Box::new(foo);
    let p_foo = &mut box_foo;

    let c = #[rustc_capture_analysis] move || p_foo.x += 10;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
// { dg-error ".E0658." "" { target *-*-* } .-4 }
// { dg-note ".E0658." "" { target *-*-* } .-5 }
// { dg-error ".E0658." "" { target *-*-* } .-6 }
// { dg-note ".E0658." "" { target *-*-* } .-7 }
}

// Test that move closures can take ownership of Copy type
fn returned_closure_owns_copy_type_data() -> impl Fn() -> i32 {
    let x = 10;

    let c = #[rustc_capture_analysis] move || x;
// { dg-error ".E0658." "" { target *-*-* } .-1 }
// { dg-note ".E0658." "" { target *-*-* } .-2 }
// { dg-note ".E0658." "" { target *-*-* } .-3 }
// { dg-error ".E0658." "" { target *-*-* } .-4 }
// { dg-note ".E0658." "" { target *-*-* } .-5 }
// { dg-error ".E0658." "" { target *-*-* } .-6 }
// { dg-note ".E0658." "" { target *-*-* } .-7 }

    c
}

fn main() {
    simple_move_closure();
    simple_ref();
    struct_contains_ref_to_another_struct_1();
    struct_contains_ref_to_another_struct_2();
    struct_contains_ref_to_another_struct_3();
    truncate_box_derefs();
    box_mut_2();
    box_mut_1();
}

