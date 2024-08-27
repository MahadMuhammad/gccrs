// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-524494170
pub struct Memory<'rom> {
    rom: &'rom [u8],
    ram: [u8; Self::SIZE],
// { dg-error "" "" { target *-*-* } .-1 }
}

impl<'rom> Memory<'rom> {
    pub const SIZE: usize = 0x8000;
}

fn main() {}

