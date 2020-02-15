#[doc = "Reader of register ISRC"]
pub type R = crate::R<u8, super::ISRC>;
#[doc = "Writer for register ISRC"]
pub type W = crate::W<u8, super::ISRC>;
#[doc = "Register ISRC `reset()`'s with value 0"]
impl crate::ResetValue for super::ISRC {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIM_A {
    #[doc = "32: 11.75 uA"]
    _11P75U = 32,
    #[doc = "16: 4.5 uA"]
    _4P5U = 16,
    #[doc = "8: 2.0 uA"]
    _2P0U = 8,
    #[doc = "4: 1.0 uA"]
    _1P0U = 4,
    #[doc = "2: 0.5 uA"]
    _0P5U = 2,
    #[doc = "1: 0.25 uA"]
    _0P25U = 1,
    #[doc = "0: No current connected"]
    NC = 0,
}
impl From<TRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TRIM`"]
pub type TRIM_R = crate::R<u8, TRIM_A>;
impl TRIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRIM_A> {
        use crate::Variant::*;
        match self.bits {
            32 => Val(TRIM_A::_11P75U),
            16 => Val(TRIM_A::_4P5U),
            8 => Val(TRIM_A::_2P0U),
            4 => Val(TRIM_A::_1P0U),
            2 => Val(TRIM_A::_0P5U),
            1 => Val(TRIM_A::_0P25U),
            0 => Val(TRIM_A::NC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_11P75U`"]
    #[inline(always)]
    pub fn is_11p75u(&self) -> bool {
        *self == TRIM_A::_11P75U
    }
    #[doc = "Checks if the value of the field is `_4P5U`"]
    #[inline(always)]
    pub fn is_4p5u(&self) -> bool {
        *self == TRIM_A::_4P5U
    }
    #[doc = "Checks if the value of the field is `_2P0U`"]
    #[inline(always)]
    pub fn is_2p0u(&self) -> bool {
        *self == TRIM_A::_2P0U
    }
    #[doc = "Checks if the value of the field is `_1P0U`"]
    #[inline(always)]
    pub fn is_1p0u(&self) -> bool {
        *self == TRIM_A::_1P0U
    }
    #[doc = "Checks if the value of the field is `_0P5U`"]
    #[inline(always)]
    pub fn is_0p5u(&self) -> bool {
        *self == TRIM_A::_0P5U
    }
    #[doc = "Checks if the value of the field is `_0P25U`"]
    #[inline(always)]
    pub fn is_0p25u(&self) -> bool {
        *self == TRIM_A::_0P25U
    }
    #[doc = "Checks if the value of the field is `NC`"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == TRIM_A::NC
    }
}
#[doc = "Write proxy for field `TRIM`"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "11.75 uA"]
    #[inline(always)]
    pub fn _11p75u(self) -> &'a mut W {
        self.variant(TRIM_A::_11P75U)
    }
    #[doc = "4.5 uA"]
    #[inline(always)]
    pub fn _4p5u(self) -> &'a mut W {
        self.variant(TRIM_A::_4P5U)
    }
    #[doc = "2.0 uA"]
    #[inline(always)]
    pub fn _2p0u(self) -> &'a mut W {
        self.variant(TRIM_A::_2P0U)
    }
    #[doc = "1.0 uA"]
    #[inline(always)]
    pub fn _1p0u(self) -> &'a mut W {
        self.variant(TRIM_A::_1P0U)
    }
    #[doc = "0.5 uA"]
    #[inline(always)]
    pub fn _0p5u(self) -> &'a mut W {
        self.variant(TRIM_A::_0P5U)
    }
    #[doc = "0.25 uA"]
    #[inline(always)]
    pub fn _0p25u(self) -> &'a mut W {
        self.variant(TRIM_A::_0P25U)
    }
    #[doc = "No current connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut W {
        self.variant(TRIM_A::NC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u8) & 0x3f) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:7 - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Current source enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:7 - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Current source enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
}
