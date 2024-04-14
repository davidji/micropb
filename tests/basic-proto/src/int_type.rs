use std::mem::size_of;

use micropb::{MessageDecode, PbDecoder};

mod proto {
    #![allow(clippy::all)]
    #![allow(warnings)]
    include!(concat!(env!("OUT_DIR"), "/int_type.rs"));
}

#[test]
fn enum_int_type() {
    let enumeration = proto::basic::Enum::One;
    let _: i8 = enumeration.0;
    assert_eq!(size_of::<proto::basic::Enum>(), 1);
}

#[test]
fn field_int_type() {
    let basic = proto::basic::BasicTypes::default();
    assert_eq!(basic.int32_num, -5i8);
    let _: i8 = basic.int32_num;
    let _: Option<&i8> = basic.int32_num();
    let _: i16 = basic.int64_num;
    let _: Option<&i16> = basic.int64_num();
    let _: u8 = basic.uint32_num;
    let _: Option<&u8> = basic.uint32_num();
    let _: u16 = basic.uint64_num;
    let _: Option<&u16> = basic.uint64_num();
    let _: i64 = basic.sfixed32_num;
    let _: Option<&i64> = basic.sfixed32_num();
    let _: isize = basic.sfixed64_num;
    let _: Option<&isize> = basic.sfixed64_num();
    let _: u64 = basic.fixed32_num;
    let _: Option<&u64> = basic.fixed32_num();
    let _: usize = basic.fixed64_num;
    let _: Option<&usize> = basic.fixed64_num();
}

#[test]
fn decode_int_overflow() {
    let mut basic = proto::basic::BasicTypes::new();
    let mut decoder = PbDecoder::new([0x03, 0x08, 0x96, 0x01].as_slice()); // field 1
    basic.decode_len_delimited(&mut decoder).unwrap();
    assert_eq!(basic.int32_num(), Some(&-106)); // 150 overflows i8

    let mut decoder = PbDecoder::new([0x03, 0x18, 0x96, 0x02].as_slice()); // field 3
    basic.decode_len_delimited(&mut decoder).unwrap();
    assert_eq!(basic.uint32_num(), Some(&22)); // 278 overflows u8
}
