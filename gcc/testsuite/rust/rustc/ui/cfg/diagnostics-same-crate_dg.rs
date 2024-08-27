#![allow(unexpected_cfgs)] // since we want to recognize them as unexpected

pub mod inner {
    #[cfg(FALSE)] // { dg-note "" "" { target *-*-* } }
    pub fn uwu() {}
// { dg-note "" "" { target *-*-* } .-1 }

    #[cfg(FALSE)] // { dg-note "" "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
    pub mod doesnt_exist {
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
        pub fn hello() {}
        pub mod hi {}
    }

    pub mod wrong {
        #[cfg(feature = "suggesting me fails the test!!")]
        pub fn meow() {}
    }

    pub mod right {
        #[cfg(feature = "what-a-cool-feature")] // { dg-note "" "" { target *-*-* } }
        pub fn meow() {}
// { dg-note "" "" { target *-*-* } .-1 }
    }
}

mod placeholder {
    use super::inner::doesnt_exist;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
// { dg-note ".E0432." "" { target *-*-* } .-2 }
    use super::inner::doesnt_exist::hi;
// { dg-error ".E0432." "" { target *-*-* } .-1 }
// { dg-note ".E0432." "" { target *-*-* } .-2 }
}

#[cfg(i_dont_exist_and_you_can_do_nothing_about_it)]
pub fn vanished() {}

fn main() {
    // There is no uwu at this path - no diagnostic.
    uwu(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-note ".E0425." "" { target *-*-* } .-1 }

    // It does exist here - diagnostic.
    inner::uwu(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-2 }

    // The module isn't found - we would like to get a diagnostic, but currently don't due to
    // the awkward way the resolver diagnostics are currently implemented.
    inner::doesnt_exist::hello(); // { dg-error ".E0433." "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-2 }

    // It should find the one in the right module, not the wrong one.
    inner::right::meow(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-note "" "" { target *-*-* } .-2 }

    // Exists in the crate root - we would generally want a diagnostic,
    // but currently don't have one.
    // Not that it matters much though, this is highly unlikely to confuse anyone.
    vanished(); // { dg-error ".E0425." "" { target *-*-* } }
// { dg-note ".E0425." "" { target *-*-* } .-1 }
}

