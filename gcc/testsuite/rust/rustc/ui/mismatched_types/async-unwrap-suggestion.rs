// { dg-additional-options "-frust-edition= 2021" }

async fn dont_suggest() -> i32 {
    if false {
        return Ok(6);
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }

    5
}

async fn do_suggest() -> i32 {
    if false {
        let s = Ok(6);
        return s;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    }

    5
}

fn main() {}

