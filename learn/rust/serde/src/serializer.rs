use super::*;
use serde::ser;
/// [serde::Serializer]
pub trait Serializer: Sized {
    /// [serde::Serializer::Ok]
    type Ok;
    /// [serde::Serializer::Error]
    type Error: ser::Error;
    /// [serde::Serializer::SerializeSeq]
    type SerializeSeq: ser::SerializeSeq<Ok = Self::Ok, Error = Self::Error>;
    /// [serde::Serializer::SerializeTuple]
    type SerializeTuple: ser::SerializeTuple<Ok = Self::Ok, Error = Self::Error>;
    /// [serde::Serializer::SerializeTupleStruct]
    type SerializeTupleStruct: ser::SerializeTupleStruct<Ok = Self::Ok, Error = Self::Error>;
    /// [serde::Serializer::SerializeTupleVariant]
    type SerializeTupleVariant: ser::SerializeTupleVariant<Ok = Self::Ok, Error = Self::Error>;
    /// [serde::Serializer::SerializeMap]
    type SerializeMap: ser::SerializeMap<Ok = Self::Ok, Error = Self::Error>;
    /// [serde::Serializer::SerializeStruct]
    type SerializeStruct: ser::SerializeStruct<Ok = Self::Ok, Error = Self::Error>;
    /// [serde::Serializer::SerializeStructVariant]
    type SerializeStructVariant: ser::SerializeStructVariant<Ok = Self::Ok, Error = Self::Error>;
    /// [serde::Serializer::serialize_bool]
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_i8]
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_i16]
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_i32]
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_i64]
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_u8]
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_u16]
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_u32]
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_u64]
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_f32]
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_f64]
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_char]
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_str]
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_bytes]
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_none]
    fn serialize_none(self) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_some]
    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize;
    /// [serde::Serializer::serialize_unit]
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_unit_struct]
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_unit_variant]
    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error>;
    /// [serde::Serializer::serialize_newtype_struct]
    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize;
    /// [serde::Serializer::serialize_newtype_variant]
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize;
    /// [serde::Serializer::serialize_seq]
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;
    /// [serde::Serializer::serialize_tuple]
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error>;
    /// [serde::Serializer::serialize_tuple_struct]
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error>;
    /// [serde::Serializer::serialize_tuple_variant]
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error>;
    /// [serde::Serializer::serialize_map]
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>;
    /// [serde::Serializer::serialize_struct]
    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error>;
    /// [serde::Serializer::serialize_struct_variant]
    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error>;
}
