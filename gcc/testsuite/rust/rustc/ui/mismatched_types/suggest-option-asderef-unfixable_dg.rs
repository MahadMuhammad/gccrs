fn produces_string() -> Option<String> {
    Some("my cool string".to_owned())
}

fn takes_str_but_too_many_refs(_: &&str) -> Option<()> {
    Some(())
}

fn no_args() -> Option<()> {
    Some(())
}

extern "C" fn takes_str_but_wrong_abi(_: &str) -> Option<()> {
    Some(())
}

unsafe fn takes_str_but_unsafe(_: &str) -> Option<()> {
    Some(())
}

struct TypeWithoutDeref;

fn main() {
    let _ = produces_string().and_then(takes_str_but_too_many_refs);
// { dg-error ".E0631." "" { target *-*-* } .-1 }
    let _ = produces_string().and_then(takes_str_but_wrong_abi);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    let _ = produces_string().and_then(takes_str_but_unsafe);
// { dg-error ".E0277." "" { target *-*-* } .-1 }
    let _ = produces_string().and_then(no_args);
// { dg-error ".E0593." "" { target *-*-* } .-1 }
    let _ = Some(TypeWithoutDeref).and_then(takes_str_but_too_many_refs);
// { dg-error ".E0631." "" { target *-*-* } .-1 }
}

