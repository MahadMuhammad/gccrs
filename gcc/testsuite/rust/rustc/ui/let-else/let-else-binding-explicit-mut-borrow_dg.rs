// Slightly different from explicit-mut-annotated -- this won't show an error until borrowck.
// Should it show a type error instead?



fn main() {
    let Some(n): &mut Option<i32> = &mut &Some(5i32) else {
// { dg-error ".E0596." "" { target *-*-* } .-1 }
        return
    };
    *n += 1;
    let _ = n;
}

