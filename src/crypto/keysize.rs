#[doc = "Reader of register KEYSIZE"]
pub type R = crate::R<u32, super::KEYSIZE>;
#[doc = "Writer for register KEYSIZE"]
pub type W = crate::W<u32, super::KEYSIZE>;
#[doc = "Register KEYSIZE `reset()`'s with value 0x01"]
impl crate::ResetValue for super::KEYSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
#[doc = "1:0\\]
Key size: 00: Reserved When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "3: 256 bits"]
    _256_BIT = 3,
    #[doc = "2: 192 bits"]
    _192_BIT = 2,
    #[doc = "1: 128 bits"]
    _128_BIT = 1,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(SIZE_A::_256_BIT),
            2 => Val(SIZE_A::_192_BIT),
            1 => Val(SIZE_A::_128_BIT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_256_BIT`"]
    #[inline(always)]
    pub fn is_256_bit(&self) -> bool {
        *self == SIZE_A::_256_BIT
    }
    #[doc = "Checks if the value of the field is `_192_BIT`"]
    #[inline(always)]
    pub fn is_192_bit(&self) -> bool {
        *self == SIZE_A::_192_BIT
    }
    #[doc = "Checks if the value of the field is `_128_BIT`"]
    #[inline(always)]
    pub fn is_128_bit(&self) -> bool {
        *self == SIZE_A::_128_BIT
    }
}
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "256 bits"]
    #[inline(always)]
    pub fn _256_bit(self) -> &'a mut W {
        self.variant(SIZE_A::_256_BIT)
    }
    #[doc = "192 bits"]
    #[inline(always)]
    pub fn _192_bit(self) -> &'a mut W {
        self.variant(SIZE_A::_192_BIT)
    }
    #[doc = "128 bits"]
    #[inline(always)]
    pub fn _128_bit(self) -> &'a mut W {
        self.variant(SIZE_A::_128_BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bits 0:1 - 1:0\\]
Key size: 00: Reserved When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bits 0:1 - 1:0\\]
Key size: 00: Reserved When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
}
