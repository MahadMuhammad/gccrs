// Checks that declaring a lang item with the wrong number of generic arguments errors rather than
// crashing (issue #83474, #83893, #87573, part of #9307, #79559).

#![feature(lang_items, no_core)]
#![no_core]

#[lang = "sized"]
trait MySized {}

#[lang = "add"]
trait MyAdd<'a, T> {}
// { dg-error ".E0718." "" { target *-*-* } .-2 }

#[lang = "drop_in_place"]
// { dg-error ".E0718." "" { target *-*-* } .-1 }
fn my_ptr_drop() {}

#[lang = "index"]
trait MyIndex<'a, T> {}
// { dg-error ".E0718." "" { target *-*-* } .-2 }

#[lang = "phantom_data"]
// { dg-error ".E0718." "" { target *-*-* } .-1 }
struct MyPhantomData<T, U>;
// { dg-error ".E0392." "" { target *-*-* } .-1 }
// { dg-error ".E0392." "" { target *-*-* } .-2 }

#[lang = "owned_box"]
// { dg-error ".E0718." "" { target *-*-* } .-1 }
struct Foo;

// When the `start` lang item is missing generics very odd things can happen, especially when
// it comes to cross-crate monomorphization
#[lang = "start"]
// { dg-error ".E0718." "" { target *-*-* } .-1 }
fn start(_: *const u8, _: isize, _: *const *const u8) -> isize {
    0
}

fn ice() {
    // Use add
    let r = 5;
    let a = 6;
    r + a; // { dg-error ".E0369." "" { target *-*-* } }

    // Use drop in place
    my_ptr_drop();

    // Use index
    let arr = [0; 5];
    let _ = arr[2];

    // Use phantomdata
    let _ = MyPhantomData::<(), i32>;

    // Use Foo
    let _: () = Foo;
}

// use `start`
fn main() {}

