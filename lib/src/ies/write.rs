use std::io::{Seek, Write};

use deku::{bitvec::BitVec, ctx::Order, prelude::*, writer::Writer};

pub(crate) fn write_bits_lsb0<W: Write + Seek>(
    writer: &mut Writer<W>,
    field: u8,
    bit_size: usize,
) -> Result<(), DekuError> {
    let bits = BitVec::from_element(field);
    writer.write_bits_order(&bits[bits.len() - bit_size..], Order::Lsb0)
}
