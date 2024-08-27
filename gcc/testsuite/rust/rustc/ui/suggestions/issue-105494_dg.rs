fn test1() {
    let _v: i32 = (1 as i32).to_string(); // { dg-error ".E0308." "" { target *-*-* } }

    // won't suggestion
    let _v: i32 = (1 as i128).to_string(); // { dg-error ".E0308." "" { target *-*-* } }

    let _v: &str = "foo".to_string(); // { dg-error ".E0308." "" { target *-*-* } }
}

fn test2() {
    let mut path: String = "/usr".to_string();
    let folder: String = "lib".to_string();

    path = format!("{}/{}", path, folder).as_str(); // { dg-error ".E0308." "" { target *-*-* } }

    println!("{}", &path);
}

fn main() {
    test1();
    test2();
}

