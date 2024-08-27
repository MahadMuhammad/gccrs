// { dg-additional-options "-frust-edition=2021" }

fn closure() {
    loop {
        let closure = || {
            if true {
                Err(1)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
            }

            Ok(())
        };
    }
}

fn async_block() {
    loop {
        let fut = async {
            if true {
                Err(1)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
// { help ".E0308." "" { target *-*-* } .-2 }
            }

            Ok(())
        };
    }
}

fn fn_item() {
    let _ = loop {
        fn foo() -> Result<(), ()> {
            if true {
                Err(1)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
            }
            Err(())
        }
    };
}

fn const_block() {
    let _ = loop {
        const {
            if true {
                Err(1)
// { dg-error ".E0308." "" { target *-*-* } .-1 }
            }
            Err(())
        };
    };
}

fn main() {}

