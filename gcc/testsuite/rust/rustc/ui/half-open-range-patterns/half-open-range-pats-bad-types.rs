fn main() {
    let "a".. = "a"; // { dg-error ".E0029." "" { target *-*-* } }
    let .."a" = "a"; // { dg-error ".E0029." "" { target *-*-* } }
    let ..="a" = "a"; // { dg-error ".E0029." "" { target *-*-* } }
}

