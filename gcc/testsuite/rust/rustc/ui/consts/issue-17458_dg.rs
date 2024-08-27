static X: usize = unsafe { core::ptr::null::<usize>() as usize };
// { dg-error "" "" { target *-*-* } .-1 }

fn main() {
    assert_eq!(X, 0);
}

