fn bug<'a>()
where
    [(); { // { dg-error ".E0308." "" { target *-*-* } }
        let _: &'a (); // { dg-error "" "" { target *-*-* } }
    }]:
{}

fn main() {}

