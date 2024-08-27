mod option {
    pub enum O<T> {
        Some(T),
        None,
    }
}

fn main() {
    let _: option::O<()> = (); // { dg-error ".E0308." "" { target *-*-* } }
}

