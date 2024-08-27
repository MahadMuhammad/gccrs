//@ run-rustfix
#![deny(rust_2021_incompatible_closure_captures)]
// { dg-note "" "" { target *-*-* } .-1 }

// Test cases for types that implement a significant drop (user defined)

#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

#[derive(Debug)]
struct ConstainsDropField(Foo, #[allow(dead_code)] Foo);

// `t` needs Drop because one of its elements needs drop,
// therefore precise capture might affect drop ordering
fn test1_all_need_migration() {
    let t = (Foo(0), Foo(0));
    let t1 = (Foo(0), Foo(0));
    let t2 = (Foo(0), Foo(0));

    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _t1 = t1.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _t2 = t2.0;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }

// String implements drop and therefore should be migrated.
// But in this test cases, `t2` is completely captured and when it is dropped won't be affected
fn test2_only_precise_paths_need_migration() {
    let t = (Foo(0), Foo(0));
    let t1 = (Foo(0), Foo(0));
    let t2 = (Foo(0), Foo(0));

    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _t1 = t1.0;
// { dg-note "" "" { target *-*-* } .-1 }
        let _t2 = t2;
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }

// If a variable would've not been captured by value then it would've not been
// dropped with the closure and therefore doesn't need migration.
fn test3_only_by_value_need_migration() {
    let t = (Foo(0), Foo(0));
    let t1 = (Foo(0), Foo(0));
    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.0;
// { dg-note "" "" { target *-*-* } .-1 }
        println!("{:?}", t1.1);
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }

// The root variable might not implement drop themselves but some path starting
// at the root variable might implement Drop.
//
// If this path isn't captured we need to migrate for the root variable.
fn test4_type_contains_drop_need_migration() {
    let t = ConstainsDropField(Foo(0), Foo(0));

    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.0;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }

// Test migration analysis in case of Drop + Non Drop aggregates.
// Note we need migration here only because the non-copy (because Drop type) is captured,
// otherwise we won't need to, since we can get away with just by ref capture in that case.
fn test5_drop_non_drop_aggregate_need_migration() {
    let t = (Foo(0), Foo(0), 0i32);

    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.0;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }

// Test migration analysis in case of Significant and Insignificant Drop aggregates.
fn test6_significant_insignificant_drop_aggregate_need_migration() {
    let t = (Foo(0), String::new());

    let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        let _t = t.1;
// { dg-note "" "" { target *-*-* } .-1 }
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }

// Since we are using a move closure here, both `t` and `t1` get moved
// even though they are being used by ref inside the closure.
fn test7_move_closures_non_copy_types_might_need_migration() {
    let t = (Foo(0), Foo(0));
    let t1 = (Foo(0), Foo(0), Foo(0));

    let c = move || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
        println!("{:?} {:?}", t1.1, t.1);
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    };

    c();
}
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }


fn test8_drop_order_and_blocks() {
    {
        let tuple =
          (Foo(0), Foo(1));
        {
            let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
                tuple.0;
// { dg-note "" "" { target *-*-* } .-1 }
            };

            c();
        }
// { dg-note "" "" { target *-*-* } .-1 }
    }
}

fn test9_drop_order_and_nested_closures() {
    let tuple =
        (Foo(0), Foo(1));
    let b = || {
        let c = || {
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
            tuple.0;
// { dg-note "" "" { target *-*-* } .-1 }
        };

        c();
    };
// { dg-note "" "" { target *-*-* } .-1 }

    b();
}

// Test that we migrate if drop order of Vec<T> would be affected if T is a significant drop type
fn test10_vec_of_significant_drop_type() {

        let tup = (Foo(0), vec![Foo(3)]);

        let _c = || tup.0;
// { dg-error "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { help "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
}
// { dg-note "" "" { target *-*-* } .-1 }

fn main() {
    test1_all_need_migration();
    test2_only_precise_paths_need_migration();
    test3_only_by_value_need_migration();
    test4_type_contains_drop_need_migration();
    test5_drop_non_drop_aggregate_need_migration();
    test6_significant_insignificant_drop_aggregate_need_migration();
    test7_move_closures_non_copy_types_might_need_migration();
    test8_drop_order_and_blocks();
    test9_drop_order_and_nested_closures();
    test10_vec_of_significant_drop_type();
}

