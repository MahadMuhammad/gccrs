fn main() {

    let y: Box<isize> = 42.into();
    let mut x: Box<isize>;

    loop {
        println!("{}", y);
        loop {
            loop {
                loop {
                    x = y; // { dg-error ".E0382." "" { target *-*-* } }
                    x.clone();
                }
            }
        }
    }
}

