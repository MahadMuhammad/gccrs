fn main() {
    fn test() {
        loop {
            let blah: Option<String>;
            if true {
                blah = Some("".to_string());
            }
            if let Some(blah) = blah.as_ref() { // { dg-error ".E0381." "" { target *-*-* } }
            }
        }
    }
    println!("{:?}", test())
}

