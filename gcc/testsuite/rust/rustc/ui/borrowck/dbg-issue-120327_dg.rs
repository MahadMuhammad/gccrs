fn s() -> String {
    let a = String::new();
    dbg!(a);
    return a; // { dg-error ".E0382." "" { target *-*-* } }
}

fn m() -> String {
    let a = String::new();
    dbg!(1, 2, a, 1, 2);
    return a; // { dg-error ".E0382." "" { target *-*-* } }
}

fn t(a: String) -> String {
    let b: String = "".to_string();
    dbg!(a, b);
    return b; // { dg-error ".E0382." "" { target *-*-* } }
}

fn x(a: String) -> String {
    let b: String = "".to_string();
    dbg!(a, b);
    return a; // { dg-error ".E0382." "" { target *-*-* } }
}

macro_rules! my_dbg {
    () => {
        eprintln!("[{}:{}:{}]", file!(), line!(), column!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                eprintln!("[{}:{}:{}] {} = {:#?}",
                    file!(), line!(), column!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(my_dbg!($val)),+,)
    };
}

fn test_my_dbg() -> String {
    let b: String = "".to_string();
    my_dbg!(b, 1);
    return b; // { dg-error ".E0382." "" { target *-*-* } }
}

fn test_not_macro() -> String {
    let a = String::new();
    let _b = match a {
        tmp => {
            eprintln!("dbg: {}", tmp);
            tmp
        }
    };
    return a; // { dg-error ".E0382." "" { target *-*-* } }
}

fn get_expr(_s: String) {}

fn test() {
    let a: String = "".to_string();
    let _res = get_expr(dbg!(a));
    let _l = a.len(); // { dg-error ".E0382." "" { target *-*-* } }
}

fn main() {}

