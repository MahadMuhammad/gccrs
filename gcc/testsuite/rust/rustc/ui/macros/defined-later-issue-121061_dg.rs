fn main() {
    something_later!(); // { dg-error "" "" { target *-*-* } }
}

macro_rules! something_later {
    () => {
        println!("successfully expanded!");
    };
}

