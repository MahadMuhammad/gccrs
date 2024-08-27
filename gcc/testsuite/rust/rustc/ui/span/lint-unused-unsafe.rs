// Exercise the unused_unsafe attribute in some positive and negative cases


// { dg-additional-options "-frust-edition=2018" }

#![allow(dead_code)]
#![deny(unused_unsafe)]


mod foo {
    extern "C" {
        pub fn bar();
    }
}

fn callback<T, F>(_f: F) -> T where F: FnOnce() -> T { panic!() }
unsafe fn unsf() {}

fn bad1() { unsafe {} }                  // { dg-error "" "" { target *-*-* } }
fn bad2() { unsafe { bad1() } }          // { dg-error "" "" { target *-*-* } }
unsafe fn bad3() { unsafe {} }           // { dg-error "" "" { target *-*-* } }
fn bad4() { unsafe { callback(||{}) } }  // { dg-error "" "" { target *-*-* } }
unsafe fn bad5() { unsafe { unsf() } }
fn bad6() {
    unsafe {                             // { dg-error "" "" { target *-*-* } }
        unsafe {                         // don't put the warning here
            unsf()
        }
    }
}
unsafe fn bad7() {
    unsafe {                             // { dg-error "" "" { target *-*-* } }
        unsafe {
            unsf()
        }
    }
}

unsafe fn good0() { unsf() }
fn good1() { unsafe { unsf() } }
fn good2() {
    /* bug uncovered when implementing warning about unused unsafe blocks. Be
       sure that when purity is inherited that the source of the unsafe-ness
       is tracked correctly */
    unsafe {
        unsafe fn what() -> Vec<String> { panic!() }

        callback(|| {
            what();
        });
    }
}

unsafe fn good3() { foo::bar() }
fn good4() { unsafe { foo::bar() } }

#[allow(unused_unsafe)] fn allowed() { unsafe {} }

fn main() {}

mod additional_tests {
    unsafe fn unsf() {}

    // some tests

    fn inner_ignored() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            #[allow(unused_unsafe)]
            unsafe {
                unsf()
            }
        }
    }

    fn multi_level_unused() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            unsafe {} // { dg-error "" "" { target *-*-* } }
            unsafe {} // { dg-error "" "" { target *-*-* } }
        }
    }

    fn granularity() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            unsafe { unsf() }
            unsafe { unsf() }
            unsafe { unsf() }
        }
    }

    fn top_level_used() {
        unsafe {
            unsf();
            unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
            unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
            unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
        }

    }

    fn top_level_ignored() {
        #[allow(unused_unsafe)]
        unsafe {
            #[deny(unused_unsafe)]
            {
                unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
                unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
                unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
            }
        }

    }

    // same tests in unsafe fn without unsafe_op_in_unsafe_fn allowed

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn inner_ignored_1() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            #[allow(unused_unsafe)]
            unsafe {
                unsf()
            }
        }
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn multi_level_unused_1() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            unsafe {} // { dg-error "" "" { target *-*-* } }
            unsafe {} // { dg-error "" "" { target *-*-* } }
        }
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn granularity_1() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            unsafe { unsf() }
            unsafe { unsf() }
            unsafe { unsf() }
        }
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_used_1() {
        unsafe {
            unsf();
            unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
            unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
            unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
        }

    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_ignored_1() {
        #[allow(unused_unsafe)]
        unsafe {
            #[deny(unused_unsafe)]
            {
                unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
                unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
                unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
            }
        }
    }

    // same tests, but unsafe_op_in_unsafe_fn allowed,
    // so that *all* unsafe blocks are unused

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn inner_ignored_2() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            #[allow(unused_unsafe)]
            unsafe {
                unsf()
            }
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn multi_level_unused_2() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            unsafe {} // { dg-error "" "" { target *-*-* } }
            unsafe {} // { dg-error "" "" { target *-*-* } }
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granularity_2() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            unsafe { unsf() }
            unsafe { unsf() }
            unsafe { unsf() }
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_used_2() {
        unsafe {
            unsf();
            unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
            unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
            unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
        }

    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_ignored_2() {
        #[allow(unused_unsafe)]
        unsafe {
            #[deny(unused_unsafe)]
            {
                unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
                unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
                unsafe { unsf() } // { dg-error "" "" { target *-*-* } }
            }
        }
    }

    // additional tests when using unsafe_op_in_unsafe_fn
    // in more complex ways

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn() {
        unsafe {
            #[deny(unsafe_op_in_unsafe_fn)]
            {
                unsf();
            }
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn_2() {
        unsafe { // { dg-error "" "" { target *-*-* } }
            unsafe {
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    unsf();
                }
            }
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn_3() {
        unsafe {
            unsafe { // { dg-error "" "" { target *-*-* } }
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    unsf();
                }
            }
            unsf();
        }
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn_4() {
        unsafe {
            unsafe { // { dg-error "" "" { target *-*-* } }
                unsf();
            }
            #[deny(unsafe_op_in_unsafe_fn)]
            {
                unsf();
            }
        }
    }
}

// the same set of tests, with closures everywhere
mod additional_tests_closures {
    unsafe fn unsf() {}

    // some tests

    fn inner_ignored() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                unsf()
            };
        };
    }

    fn multi_level_unused() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
        };
    }

    fn granularity() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { unsf() };
            let _ = || unsafe { unsf() };
            let _ = || unsafe { unsf() };
        };
    }

    fn top_level_used() {
        let _ = || unsafe {
            unsf();
            let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
        };

    }

    fn top_level_ignored() {
        #[allow(unused_unsafe)]
        let _ = || unsafe {
            #[deny(unused_unsafe)]
            {
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            }
        };

    }

    // same tests in unsafe fn without unsafe_op_in_unsafe_fn allowed

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn inner_ignored_1() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                unsf()
            };
        };
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn multi_level_unused_1() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
        };
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn granularity_1() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { unsf() };
            let _ = || unsafe { unsf() };
            let _ = || unsafe { unsf() };
        };
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_used_1() {
        let _ = || unsafe {
            unsf();
            let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
        };

    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_ignored_1() {
        #[allow(unused_unsafe)]
        let _ = || unsafe {
            #[deny(unused_unsafe)]
            {
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            }
        };
    }

    // same tests, but unsafe_op_in_unsafe_fn allowed,
    // so that *all* unsafe blocks are unused

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn inner_ignored_2() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                unsf()
            };
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn multi_level_unused_2() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granularity_2() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { unsf() };
            let _ = || unsafe { unsf() };
            let _ = || unsafe { unsf() };
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_used_2() {
        let _ = || unsafe {
            unsf();
            let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
        };

    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_ignored_2() {
        #[allow(unused_unsafe)]
        let _ = || unsafe {
            #[deny(unused_unsafe)]
            {
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            }
        };
    }

    // additional tests when using unsafe_op_in_unsafe_fn
    // in more complex ways

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn() {
        let _ = || unsafe {
            #[deny(unsafe_op_in_unsafe_fn)]
            {
                unsf();
            }
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn_2() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    unsf();
                }
            };
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn_3() {
        let _ = || unsafe {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    unsf();
                }
            };
            unsf();
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn_4() {
        let _ = || unsafe {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                unsf();
            };
            #[deny(unsafe_op_in_unsafe_fn)]
            {
                unsf();
            }
        };
    }
}

// the same set of tests, with closures everywhere
// and closures on the unsafe fn calls
mod additional_tests_even_more_closures {
    unsafe fn unsf() {}

    // some tests

    fn inner_ignored() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                let _ = || unsf();
            };
        };
    }

    fn multi_level_unused() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
        };
    }

    fn granularity() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { let _ = || unsf(); };
            let _ = || unsafe { let _ = || unsf(); };
            let _ = || unsafe { let _ = || unsf(); };
        };
    }

    fn top_level_used() {
        let _ = || unsafe {
            let _ = || unsf();
            let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
        };

    }

    fn top_level_ignored() {
        #[allow(unused_unsafe)]
        let _ = || unsafe {
            #[deny(unused_unsafe)]
            {
                let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
            }
        };

    }

    // same tests in unsafe fn without unsafe_op_in_unsafe_fn allowed

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn inner_ignored_1() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                let _ = || unsf();
            };
        };
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn multi_level_unused_1() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
        };
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn granularity_1() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { let _ = || unsf(); };
            let _ = || unsafe { let _ = || unsf(); };
            let _ = || unsafe { let _ = || unsf(); };
        };
    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_used_1() {
        let _ = || unsafe {
            let _ = || unsf();
            let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
        };

    }

    #[deny(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_ignored_1() {
        #[allow(unused_unsafe)]
        let _ = || unsafe {
            #[deny(unused_unsafe)]
            {
                let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
            }
        };
    }

    // same tests, but unsafe_op_in_unsafe_fn allowed,
    // so that *all* unsafe blocks are unused

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn inner_ignored_2() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                let _ = || unsf();
            };
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn multi_level_unused_2() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granularity_2() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { let _ = || unsf(); };
            let _ = || unsafe { let _ = || unsf(); };
            let _ = || unsafe { let _ = || unsf(); };
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_used_2() {
        let _ = || unsafe {
            let _ = || unsf();
            let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
        };

    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn top_level_ignored_2() {
        #[allow(unused_unsafe)]
        let _ = || unsafe {
            #[deny(unused_unsafe)]
            {
                let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { let _ = || unsf(); }; // { dg-error "" "" { target *-*-* } }
            }
        };
    }

    // additional tests when using unsafe_op_in_unsafe_fn
    // in more complex ways

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn() {
        let _ = || unsafe {
            #[deny(unsafe_op_in_unsafe_fn)]
            {
                let _ = || unsf();
            }
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn_2() {
        let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
            let _ = || unsafe {
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    let _ = || unsf();
                }
            };
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn_3() {
        let _ = || unsafe {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    let _ = || unsf();
                }
            };
            let _ = || unsf();
        };
    }

    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn_4() {
        let _ = || unsafe {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsf();
            };
            #[deny(unsafe_op_in_unsafe_fn)]
            {
                let _ = || unsf();
            }
        };
    }
}

mod item_likes {
    unsafe fn unsf() {}

    struct S;
    impl S {
        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn inner_ignored_1() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                #[allow(unused_unsafe)]
                let _ = || unsafe {
                    unsf()
                };
            };
        }

        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn multi_level_unused_1() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            };
        }

        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn granularity_1() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() };
                let _ = || unsafe { unsf() };
                let _ = || unsafe { unsf() };
            };
        }

        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn top_level_used_1() {
            let _ = || unsafe {
                unsf();
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            };

        }

        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn top_level_ignored_1() {
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                #[deny(unused_unsafe)]
                {
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                }
            };
        }

        // same tests, but unsafe_op_in_unsafe_fn allowed,
        // so that *all* unsafe blocks are unused

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn inner_ignored_2() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                #[allow(unused_unsafe)]
                let _ = || unsafe {
                    unsf()
                };
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn multi_level_unused_2() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granularity_2() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() };
                let _ = || unsafe { unsf() };
                let _ = || unsafe { unsf() };
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn top_level_used_2() {
            let _ = || unsafe {
                unsf();
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            };

        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn top_level_ignored_2() {
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                #[deny(unused_unsafe)]
                {
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                }
            };
        }

        // additional tests when using unsafe_op_in_unsafe_fn
        // in more complex ways

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granular_disallow_op_in_unsafe_fn() {
            let _ = || unsafe {
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    unsf();
                }
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granular_disallow_op_in_unsafe_fn_2() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {
                    #[deny(unsafe_op_in_unsafe_fn)]
                    {
                        unsf();
                    }
                };
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granular_disallow_op_in_unsafe_fn_3() {
            let _ = || unsafe {
                let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                    #[deny(unsafe_op_in_unsafe_fn)]
                    {
                        unsf();
                    }
                };
                unsf();
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granular_disallow_op_in_unsafe_fn_4() {
            let _ = || unsafe {
                let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                    unsf();
                };
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    unsf();
                }
            };
        }
    }

    trait T {
        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn inner_ignored_1() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                #[allow(unused_unsafe)]
                let _ = || unsafe {
                    unsf()
                };
            };
        }

        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn multi_level_unused_1() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            };
        }

        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn granularity_1() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() };
                let _ = || unsafe { unsf() };
                let _ = || unsafe { unsf() };
            };
        }

        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn top_level_used_1() {
            let _ = || unsafe {
                unsf();
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            };

        }

        #[deny(unsafe_op_in_unsafe_fn)]
        unsafe fn top_level_ignored_1() {
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                #[deny(unused_unsafe)]
                {
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                }
            };
        }

        // same tests, but unsafe_op_in_unsafe_fn allowed,
        // so that *all* unsafe blocks are unused

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn inner_ignored_2() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                #[allow(unused_unsafe)]
                let _ = || unsafe {
                    unsf()
                };
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn multi_level_unused_2() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {}; // { dg-error "" "" { target *-*-* } }
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granularity_2() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() };
                let _ = || unsafe { unsf() };
                let _ = || unsafe { unsf() };
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn top_level_used_2() {
            let _ = || unsafe {
                unsf();
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
            };

        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn top_level_ignored_2() {
            #[allow(unused_unsafe)]
            let _ = || unsafe {
                #[deny(unused_unsafe)]
                {
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                    let _ = || unsafe { unsf() }; // { dg-error "" "" { target *-*-* } }
                }
            };
        }

        // additional tests when using unsafe_op_in_unsafe_fn
        // in more complex ways

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granular_disallow_op_in_unsafe_fn() {
            let _ = || unsafe {
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    unsf();
                }
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granular_disallow_op_in_unsafe_fn_2() {
            let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = || unsafe {
                    #[deny(unsafe_op_in_unsafe_fn)]
                    {
                        unsf();
                    }
                };
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granular_disallow_op_in_unsafe_fn_3() {
            let _ = || unsafe {
                let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                    #[deny(unsafe_op_in_unsafe_fn)]
                    {
                        unsf();
                    }
                };
                unsf();
            };
        }

        #[allow(unsafe_op_in_unsafe_fn)]
        unsafe fn granular_disallow_op_in_unsafe_fn_4() {
            let _ = || unsafe {
                let _ = || unsafe { // { dg-error "" "" { target *-*-* } }
                    unsf();
                };
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    unsf();
                }
            };
        }
    }
}

mod additional_tests_extra {
    unsafe fn unsf() {}

    // multiple uses with different `unsafe_op_in_unsafe_fn` in the same closure
    #[allow(unsafe_op_in_unsafe_fn)]
    unsafe fn granular_disallow_op_in_unsafe_fn() {
        let _ = || unsafe {
            let _ = || {
                unsf();
                #[deny(unsafe_op_in_unsafe_fn)]
                {
                    unsf();
                }
            };
        };
    }

    #[warn(unsafe_op_in_unsafe_fn)]
    unsafe fn multiple_unsafe_op_in_unsafe_fn_allows() {
        unsafe {
            #[allow(unsafe_op_in_unsafe_fn)]
            {
                unsf();
            }
            #[allow(unsafe_op_in_unsafe_fn)]
            {
                unsf();
            }
        }
    }

    async unsafe fn async_blocks() {
        #[deny(unsafe_op_in_unsafe_fn)]
        {
            let _ = async { unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = async { unsafe { let _ = async { unsf() }; }};
                let _ = async { unsafe { let _ = async { unsf() }; }};
                let _ = async { unsafe { let _ = async { unsf() }; }};
            }};
            let _ = async { unsafe {
                let _ = async { unsf() };
                let _ = async { unsafe { let _ = async { unsf() }; }}; // { dg-error "" "" { target *-*-* } }
                let _ = async { unsafe { let _ = async { unsf() }; }}; // { dg-error "" "" { target *-*-* } }
                let _ = async { unsafe { let _ = async { unsf() }; }}; // { dg-error "" "" { target *-*-* } }
            }};
        }
        #[allow(unsafe_op_in_unsafe_fn)]
        {
            let _ = async { unsafe { // { dg-error "" "" { target *-*-* } }
                let _ = async { unsafe { let _ = async { unsf() }; }};
                let _ = async { unsafe { let _ = async { unsf() }; }};
                let _ = async { unsafe { let _ = async { unsf() }; }};
            }};
            let _ = async { unsafe {
                let _ = async { unsf() };
                let _ = async { unsafe { let _ = async { unsf() }; }}; // { dg-error "" "" { target *-*-* } }
                let _ = async { unsafe { let _ = async { unsf() }; }}; // { dg-error "" "" { target *-*-* } }
                let _ = async { unsafe { let _ = async { unsf() }; }}; // { dg-error "" "" { target *-*-* } }
            }};
        }
    }

    fn used_unsafe_in_const() {
        let _x: [(); unsafe { size() }] = [];
    }

    fn unused_unsafe_in_const_1() {
        let _x: [(); unsafe { 0 }] = []; // { dg-error "" "" { target *-*-* } }
    }

    fn unused_unsafe_in_const_2() {
        let _x: [(); unsafe { unsafe { size() } }] = []; // { dg-error "" "" { target *-*-* } }
    }

    const unsafe fn size() -> usize { 0 }
}

