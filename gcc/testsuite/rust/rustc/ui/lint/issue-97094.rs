#![deny(warnings)]

// Ensure that unknown lints inside cfg-attr's are linted for

#![cfg_attr(all(), allow(nonex_lint_top_level))]
// { dg-error "" "" { target *-*-* } .-1 }
#![cfg_attr(all(), allow(bare_trait_object))]
// { dg-error "" "" { target *-*-* } .-1 }

#[cfg_attr(all(), allow(nonex_lint_mod))]
// { dg-error "" "" { target *-*-* } .-1 }
mod baz {
    #![cfg_attr(all(), allow(nonex_lint_mod_inner))]
// { dg-error "" "" { target *-*-* } .-1 }
}

#[cfg_attr(all(), allow(nonex_lint_fn))]
// { dg-error "" "" { target *-*-* } .-1 }
pub fn main() {}

macro_rules! bar {
    ($($t:tt)*) => {
        $($t)*
    };
}

bar!(
    #[cfg_attr(all(), allow(nonex_lint_in_macro))]
// { dg-error "" "" { target *-*-* } .-1 }
    pub fn _bar() {}
);

// No warning for non-applying cfg
#[cfg_attr(any(), allow(nonex_lint_fn))]
pub fn _foo() {}

// Allowing unknown lints works if inside cfg_attr
#[cfg_attr(all(), allow(unknown_lints))]
mod bar_allowed {
    #[allow(nonex_lint_fn)]
    fn _foo() {}
}

// ... but not if the cfg_attr doesn't evaluate
#[cfg_attr(any(), allow(unknown_lints))]
mod bar_not_allowed {
    #[allow(nonex_lint_fn)]
// { dg-error "" "" { target *-*-* } .-1 }
    fn _foo() {}
}

