fn main() {
    let number = 2;
    Some(true).filter({ // { dg-error ".E0277." "" { target *-*-* } }
        if number % 2 == 0 {
            number == 0
        } else {
            number != 0
        }
    });
}

