#[derive(Debug)]
struct NotCopyable;
#[derive(Debug, Clone)]
struct NotCopyableButCloneable;

fn func<F: FnMut() -> H, H: FnMut()>(_: F) {}

fn foo() {
    let var = NotCopyable;
    func(|| {
        // Shouldn't suggest `move ||.as_ref()` here
        move || { // { dg-error ".E0507." "" { target *-*-* } }
            let x = var; // { dg-error ".E0507." "" { target *-*-* } }
            println!("{x:?}");
        }
    });
}

fn bar() {
    let var = NotCopyableButCloneable;
    func(|| {
        // Shouldn't suggest `move ||.as_ref()` here
        move || { // { dg-error ".E0507." "" { target *-*-* } }
            let x = var; // { dg-error ".E0507." "" { target *-*-* } }
            println!("{x:?}");
        }
    });
}

fn main() {}

