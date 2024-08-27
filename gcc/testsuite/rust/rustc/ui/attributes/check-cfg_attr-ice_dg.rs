//! I missed a `cfg_attr` match in #128581, it should have had the same treatment as `cfg`. If
//! an invalid attribute starting with `cfg_attr` is passed, then it would trigger an ICE because
//! it was not considered "checked" (e.g. `#[cfg_attr::skip]` or `#[cfg_attr::no_such_thing]`).
//!
//! This test is not exhaustive, there's too many possible positions to check, instead it just does
//! a basic smoke test in a few select positions to make sure we don't ICE for e.g.
//! `#[cfg_attr::no_such_thing]`.
//!
//! issue: rust-lang/rust#128716
#![crate_type = "lib"]

#[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
mod we_are_no_strangers_to_love {}

#[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
struct YouKnowTheRules {
    #[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    and_so_do_i: u8,
}

#[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
fn a_full_commitment() {
    #[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    let is_what_i_am_thinking_of = ();
}

#[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
union AnyOtherGuy {
    owo: ()
}
struct This;

#[cfg_attr(FALSE, doc = "you wouldn't get this")]
impl From<AnyOtherGuy> for This {
    #[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    fn from(#[cfg_attr::no_such_thing] any_other_guy: AnyOtherGuy) -> This {
// { dg-error ".E0433." "" { target *-*-* } .-1 }
        #[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
// { dg-error ".E0433." "" { target *-*-* } .-2 }
        unreachable!()
    }
}

#[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
enum NeverGonna {
    #[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    GiveYouUp(#[cfg_attr::no_such_thing] u8),
// { dg-error ".E0433." "" { target *-*-* } .-1 }
    LetYouDown {
        #![cfg_attr::no_such_thing]
// { dg-error "" "" { target *-*-* } .-1 }
        never_gonna: (),
        round_around: (),
        #[cfg_attr::no_such_thing]
// { dg-error ".E0433." "" { target *-*-* } .-1 }
        and_desert_you: (),
    },
}

