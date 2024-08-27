// Issue https://github.com/rust-lang/rust/issues/123414
trait MemoryUnit {
    extern "C" fn read_word(&mut self) -> u8;
    extern "C" fn read_dword(Self::Assoc<'_>) -> u16;
// { dg-warning ".E0220." "" { target *-*-* } .-1 }
// { dg-warning ".E0220." "" { target *-*-* } .-2 }
// { dg-error ".E0220." "" { target *-*-* } .-3 }
}

struct ROM {}

impl MemoryUnit for ROM {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
    extern "C" fn read_dword(&'_ self) -> u16 {
// { dg-error ".E0185." "" { target *-*-* } .-1 }
        let a16 = self.read_word() as u16;
// { dg-error ".E0596." "" { target *-*-* } .-1 }
        let b16 = self.read_word() as u16;
// { dg-error ".E0596." "" { target *-*-* } .-1 }

        (b16 << 8) | a16
    }
}

pub fn main() {}

