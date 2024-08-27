#![feature(if_let_guard)]

fn foo(_:String) {}

fn main()
{
    let my_str = "hello".to_owned();
    match Some(42) {
        Some(_) if { drop(my_str); false } => {}
        Some(_) => {}
        None => { foo(my_str); } // { dg-error ".E0382." "" { target *-*-* } }
    }

    let my_str = "hello".to_owned();
    match Some(42) {
        Some(_) if let Some(()) = { drop(my_str); None } => {}
        Some(_) => {}
        None => { foo(my_str); } // { dg-error ".E0382." "" { target *-*-* } }
    }
}

