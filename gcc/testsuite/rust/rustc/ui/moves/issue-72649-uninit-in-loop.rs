// Regression test for issue #72649
// Tests that we don't emit spurious
// 'value moved in previous iteration of loop' message

struct NonCopy;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
// { dg-note "" "" { target *-*-* } .-3 }
// { dg-note "" "" { target *-*-* } .-4 }
// { dg-note "" "" { target *-*-* } .-5 }
// { dg-note "" "" { target *-*-* } .-6 }
// { dg-note "" "" { target *-*-* } .-7 }
// { dg-note "" "" { target *-*-* } .-8 }

fn good() {
    loop {
        let value = NonCopy{};
        let _used = value;
    }
}

fn moved_here_1() {
    loop {
        let value = NonCopy{};
// { dg-note "" "" { target *-*-* } .-1 }
        let _used = value;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        let _used2 = value; // { dg-error ".E0382." "" { target *-*-* } }
// { dg-note ".E0382." "" { target *-*-* } .-1 }
    }
}

fn moved_here_2() {
    let value = NonCopy{};
// { dg-note "" "" { target *-*-* } .-1 }
    loop { // { dg-note "" "" { target *-*-* } }
        let _used = value;
// { dg-note "" "" { target *-*-* } .-1 }
// { dg-note "" "" { target *-*-* } .-2 }
        loop {
            let _used2 = value; // { dg-error ".E0382." "" { target *-*-* } }
// { dg-note ".E0382." "" { target *-*-* } .-1 }
        }
    }
}

fn moved_loop_1() {
    let value = NonCopy{};
// { dg-note "" "" { target *-*-* } .-1 }
    loop { // { dg-note "" "" { target *-*-* } }
        let _used = value; // { dg-error ".E0382." "" { target *-*-* } }
// { dg-note ".E0382." "" { target *-*-* } .-1 }
// { dg-note ".E0382." "" { target *-*-* } .-2 }
    }
}

fn moved_loop_2() {
    let mut value = NonCopy{};
// { dg-note "" "" { target *-*-* } .-1 }
    let _used = value;
    value = NonCopy{};
    loop { // { dg-note "" "" { target *-*-* } }
        let _used2 = value; // { dg-error ".E0382." "" { target *-*-* } }
// { dg-note ".E0382." "" { target *-*-* } .-1 }
// { dg-note ".E0382." "" { target *-*-* } .-2 }
    }
}

fn uninit_1() {
    loop {
        let value: NonCopy; // { dg-note "" "" { target *-*-* } }
        let _used = value; // { dg-error ".E0381." "" { target *-*-* } }
// { dg-note ".E0381." "" { target *-*-* } .-1 }
    }
}

fn uninit_2() {
    let mut value: NonCopy; // { dg-note "" "" { target *-*-* } }
    loop {
        let _used = value; // { dg-error ".E0381." "" { target *-*-* } }
// { dg-note ".E0381." "" { target *-*-* } .-1 }
    }
}

fn main() {}

