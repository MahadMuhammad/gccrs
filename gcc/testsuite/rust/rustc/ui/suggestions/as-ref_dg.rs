struct Foo;

fn takes_ref(_: &Foo) {}

fn main() {
    let ref opt = Some(Foo);
    opt.map(|arg| takes_ref(arg)); // { dg-error ".E0308." "" { target *-*-* } }
    opt.and_then(|arg| Some(takes_ref(arg))); // { dg-error ".E0308." "" { target *-*-* } }
    let ref opt: Result<_, ()> = Ok(Foo);
    opt.map(|arg| takes_ref(arg)); // { dg-error ".E0308." "" { target *-*-* } }
    opt.and_then(|arg| Ok(takes_ref(arg))); // { dg-error ".E0308." "" { target *-*-* } }
    let x: &Option<usize> = &Some(3);
    let y: Option<&usize> = x; // { dg-error ".E0308." "" { target *-*-* } }
    let x: &Result<usize, usize> = &Ok(3);
    let y: Result<&usize, &usize> = x;
// { dg-error ".E0308." "" { target *-*-* } .-1 }
    // note: do not suggest because of `E: usize`
    let x: &Result<usize, usize> = &Ok(3);
    let y: Result<&usize, usize> = x; // { dg-error ".E0308." "" { target *-*-* } }

    let multiple_ref_opt = &&Some(Foo);
    multiple_ref_opt.map(|arg| takes_ref(arg)); // { dg-error ".E0308." "" { target *-*-* } }
    multiple_ref_opt.and_then(|arg| Some(takes_ref(arg))); // { dg-error ".E0308." "" { target *-*-* } }
    let multiple_ref_result = &&Ok(Foo);
    multiple_ref_result.map(|arg| takes_ref(arg)); // { dg-error ".E0308." "" { target *-*-* } }
    multiple_ref_result.and_then(|arg| Ok(takes_ref(arg))); // { dg-error ".E0308." "" { target *-*-* } }

    let _: Result<&usize, _> = &Ok(42); // { dg-error ".E0308." "" { target *-*-* } }
}

