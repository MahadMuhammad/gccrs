fn foo() -> Result<String, ()> {
    let out: Result<(), ()> = Ok(());
    out // { dg-error ".E0308." "" { target *-*-* } }
}

fn main() {}

