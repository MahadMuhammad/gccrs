#[derive(Debug)]
enum MyError {
    MainError
}

fn main() -> Result<(), MyError> {
    let vec = vec!["one", "two", "three"];
    let list = vec
        .iter()
        .map(|s| s.strip_prefix("t"))
        .filter_map(Option::Some)
        .into()?; // { dg-error ".E0283." "" { target *-*-* } }

    return Ok(());
}

