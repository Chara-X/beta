use std::char;
/// [std::primitive::char]
pub struct Char;
impl Char {
    /// [std::primitive::char::MIN]
    pub const MIN: char = '\0';
    /// [std::primitive::char::MAX]
    pub const MAX: char = '\u{10ffff}';
    /// [std::primitive::char::REPLACEMENT_CHARACTER]
    pub const REPLACEMENT_CHARACTER: char = '�';
    /// [std::primitive::char::decode_utf16]
    pub fn decode_utf16<I>(iter: I) -> char::DecodeUtf16<<I as IntoIterator>::IntoIter>
    where
        I: IntoIterator<Item = u16>,
    {
        todo!()
    }
    /// [std::primitive::char::escape_unicode]
    pub fn escape_unicode(self) -> char::EscapeUnicode {
        todo!()
    }
    /// [std::primitive::char::is_alphabetic]
    pub fn is_alphabetic(self) -> bool {
        todo!()
    }
    /// [std::primitive::char::is_numeric]
    pub fn is_numeric(self) -> bool {
        todo!()
    }
    /// [std::primitive::char::is_alphanumeric]
    pub fn is_alphanumeric(self) -> bool {
        todo!()
    }
    /// [std::primitive::char::is_whitespace]
    pub fn is_whitespace(self) -> bool {
        todo!()
    }
    /// [std::primitive::char::is_control]
    pub fn is_control(self) -> bool {
        todo!()
    }
    /// [std::primitive::char::is_lowercase]
    pub const fn is_lowercase(self) -> bool {
        todo!()
    }
    /// [std::primitive::char::is_uppercase]
    pub const fn is_uppercase(self) -> bool {
        todo!()
    }
    /// [std::primitive::char::to_lowercase]
    pub fn to_lowercase(self) -> char::ToLowercase {
        todo!()
    }
    /// [std::primitive::char::to_uppercase]
    pub fn to_uppercase(self) -> char::ToUppercase {
        todo!()
    }
    /// [std::primitive::char::len_utf8]
    pub const fn len_utf8(self) -> usize {
        todo!()
    }
    /// [std::primitive::char::len_utf16]
    pub const fn len_utf16(self) -> usize {
        todo!()
    }
    /// [std::primitive::char::encode_utf8]
    pub const fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
        todo!()
    }
    /// [std::primitive::char::encode_utf16]
    pub const fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
        todo!()
    }
}
