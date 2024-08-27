const unsafe fn require_unsafe() -> usize {
    1
}

fn main() {
    const {
        require_unsafe();
// { dg-error ".E0133." "" { target *-*-* } .-1 }
    }
}

