// test for #112824 ICE type mismatching when copying!

pub struct Opcode(pub u8);

pub struct Opcode2(&'a S);
// { dg-error ".E0412." "" { target *-*-* } .-1 }
// { dg-error ".E0412." "" { target *-*-* } .-2 }

impl Opcode2 {
    pub const OP2: Opcode2 = Opcode2(Opcode(0x1));
}

pub fn example2(msg_type: Opcode2) -> impl FnMut(&[u8]) {
    move |i| match msg_type {
        Opcode2::OP2 => unimplemented!(),
// { dg-error "" "" { target *-*-* } .-1 }
    }
}

pub fn main() {}

