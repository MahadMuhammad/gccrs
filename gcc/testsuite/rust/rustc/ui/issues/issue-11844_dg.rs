fn main() {
    let a = Some(Box::new(1));
    match a {
        Ok(a) => // { dg-error ".E0308." "" { target *-*-* } }
            println!("{}",a),
        None => panic!()
    }
}

