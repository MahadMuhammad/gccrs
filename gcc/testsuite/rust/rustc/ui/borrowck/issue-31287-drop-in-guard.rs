#![feature(if_let_guard)]

fn main() {
    let a = Some("...".to_owned());
    let b = match a {
        Some(_) if { drop(a); false } => None,
        x => x, // { dg-error ".E0382." "" { target *-*-* } }
    };

    let a = Some("...".to_owned());
    let b = match a {
        Some(_) if let Some(()) = { drop(a); None } => None,
        x => x, // { dg-error ".E0382." "" { target *-*-* } }
    };
}

