// this isn't auto-fixable now because we produce two similar suggestions

fn produces_string() -> Option<String> {
    Some("my cool string".to_owned())
}

fn takes_str(_: &str) -> Option<()> {
    Some(())
}

fn takes_str_mut(_: &mut str) -> Option<()> {
    Some(())
}

fn generic<T>(_: T) -> Option<()> {
    Some(())
}

fn generic_ref<T>(_: &T) -> Option<()> {
// { help "" "" { target *-*-* } .-1 }
    Some(())
}

fn main() {
    let _: Option<()> = produces_string().and_then(takes_str);
// { dg-error ".E0631." "" { target *-*-* } .-1 }
// { help ".E0631." "" { target *-*-* } .-2 }
// { help ".E0631." "" { target *-*-* } .-3 }
    let _: Option<Option<()>> = produces_string().map(takes_str);
// { dg-error ".E0631." "" { target *-*-* } .-1 }
// { help ".E0631." "" { target *-*-* } .-2 }
// { help ".E0631." "" { target *-*-* } .-3 }
    let _: Option<Option<()>> = produces_string().map(takes_str_mut);
// { dg-error ".E0631." "" { target *-*-* } .-1 }
// { help ".E0631." "" { target *-*-* } .-2 }
// { help ".E0631." "" { target *-*-* } .-3 }
    let _ = produces_string().and_then(generic);

    let _ = produces_string().and_then(generic_ref);
// { dg-error ".E0631." "" { target *-*-* } .-1 }
// { help ".E0631." "" { target *-*-* } .-2 }
// { help ".E0631." "" { target *-*-* } .-3 }
}

