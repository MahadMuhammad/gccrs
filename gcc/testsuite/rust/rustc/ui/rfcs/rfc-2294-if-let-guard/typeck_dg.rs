#![feature(if_let_guard)]

fn ok() -> Result<Option<bool>, ()> {
    Ok(Some(true))
}

fn main() {
    match ok() {
        Ok(x) if let Err(_) = x => {},
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        Ok(x) if let 0 = x => {},
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        _ => {}
    }
}

