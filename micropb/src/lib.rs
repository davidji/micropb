#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

use num_traits::{AsPrimitive, PrimInt, Zero};

pub mod callback;
pub mod container;
#[cfg(feature = "decode")]
pub mod decode;
#[cfg(feature = "encode")]
pub mod encode;
pub mod extension;
pub mod message;
mod misc;
#[cfg(feature = "encode")]
pub mod size;

pub use container::{PbContainer, PbMap, PbString, PbVec};
#[cfg(feature = "decode")]
pub use decode::{DecodeError, DecodeFixedSize, PbDecoder, PbRead};
#[cfg(feature = "encode")]
pub use encode::{PbEncoder, PbWrite};

pub const WIRE_TYPE_VARINT: u8 = 0;
pub const WIRE_TYPE_I64: u8 = 1;
pub const WIRE_TYPE_LEN: u8 = 2;
pub const WIRE_TYPE_I32: u8 = 5;

#[derive(Debug, PartialEq, Clone, Copy)]
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

pub trait ImplicitPresence: sealed::Sealed {
    fn pb_is_present(&self) -> bool;
}

macro_rules! impl_implicit_presence_num {
    ($typ:ty) => {
        impl ImplicitPresence for $typ {
            fn pb_is_present(&self) -> bool {
                !self.is_zero()
            }
        }
    };
}

impl_implicit_presence_num!(u32);
impl_implicit_presence_num!(i32);
impl_implicit_presence_num!(u64);
impl_implicit_presence_num!(i64);
impl_implicit_presence_num!(f32);
impl_implicit_presence_num!(f64);

impl ImplicitPresence for bool {
    fn pb_is_present(&self) -> bool {
        *self
    }
}

impl ImplicitPresence for str {
    fn pb_is_present(&self) -> bool {
        !self.is_empty()
    }
}

impl ImplicitPresence for [u8] {
    fn pb_is_present(&self) -> bool {
        !self.is_empty()
    }
}

impl<T: ImplicitPresence> ImplicitPresence for &T {
    fn pb_is_present(&self) -> bool {
        (*self).pb_is_present()
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for u32 {}
    impl Sealed for i32 {}
    impl Sealed for u64 {}
    impl Sealed for i64 {}
    impl Sealed for f32 {}
    impl Sealed for f64 {}
    impl Sealed for bool {}
    impl Sealed for str {}
    impl Sealed for [u8] {}
    impl<T: Sealed> Sealed for &T {}
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