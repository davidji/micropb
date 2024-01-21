#![cfg_attr(not(test), no_std)]

use num_traits::{AsPrimitive, PrimInt};

pub mod container;
pub mod decode;
pub mod encode;
pub mod message;
pub mod size;

pub const WIRE_TYPE_VARINT: u8 = 0;
pub const WIRE_TYPE_I64: u8 = 1;
pub const WIRE_TYPE_LEN: u8 = 2;
pub const WIRE_TYPE_I32: u8 = 5;

#[derive(Debug, PartialEq)]
pub struct Tag(u32);

impl Tag {
    #[inline]
    pub fn from_parts(field_num: u32, wire_type: u8) -> Self {
        debug_assert!(wire_type <= 7);
        Self((field_num << 3) | (wire_type as u32))
    }

    #[inline]
    pub fn wire_type(&self) -> u8 {
        (self.0 & 0b111) as u8
    }

    #[inline]
    pub fn field_num(&self) -> u32 {
        self.0 >> 3
    }

    #[inline]
    pub fn varint(&self) -> u32 {
        self.0
    }
}

trait VarInt: PrimInt + From<u8> + AsPrimitive<u8> {
    const BYTES: u8;
}

impl VarInt for u32 {
    const BYTES: u8 = 5;
}

impl VarInt for u64 {
    const BYTES: u8 = 10;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag() {
        let tag = Tag::from_parts(5, 4);
        assert_eq!(tag.varint(), 0x2C);
        assert_eq!(tag.field_num(), 5);
        assert_eq!(tag.wire_type(), 4);

        let tag = Tag::from_parts(0, 0);
        assert_eq!(tag.varint(), 0);
        assert_eq!(tag.field_num(), 0);
        assert_eq!(tag.wire_type(), 0);
    }
}
