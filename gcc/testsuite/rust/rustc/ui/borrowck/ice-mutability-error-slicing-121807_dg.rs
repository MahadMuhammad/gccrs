// { dg-additional-options "-frust-edition=2015" }
// test for ICE #121807 begin <= end (12 <= 11) when slicing 'Self::Assoc<'_>'
// fixed by #122749

trait MemoryUnit { // ERROR: not all trait items implemented, missing: `read_word`
    extern "C" fn read_word(&mut self) -> u8;
    extern "C" fn read_dword(Self::Assoc<'_>) -> u16;
// { dg-warning ".E0220." "" { target *-*-* } .-1 }
// { dg-warning ".E0220." "" { target *-*-* } .-2 }
// { dg-error ".E0220." "" { target *-*-* } .-3 }
}

struct ROM {}

impl MemoryUnit for ROM {
// { dg-error ".E0046." "" { target *-*-* } .-1 }
    extern "C" fn read_dword(&'s self) -> u16 {
// { dg-error ".E0185." "" { target *-*-* } .-1 }
// { dg-error ".E0185." "" { target *-*-* } .-2 }
        let a16 = self.read_word() as u16;
        let b16 = self.read_word() as u16;

        (b16 << 8) | a16
    }
}

pub fn main() {}

