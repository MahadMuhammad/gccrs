struct Qux(i32);

fn bad() {
    let mut map = std::collections::HashMap::new();
    map.insert(('a', 'b'), ('c', 'd'));

    for ((_, _), (&mut c, _)) in &mut map {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        if c == 'e' {}
    }
}

fn bad2() {
    for Some(Qux(_)) | None in [Some(""), None] {
// { dg-error ".E0308." "" { target *-*-* } .-1 }
        todo!();
    }
}

fn main() {}

