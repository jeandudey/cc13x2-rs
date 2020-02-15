#[doc = "Reader of register STCR"]
pub type R = crate::R<u32, super::STCR>;
#[doc = "Writer for register STCR"]
pub type W = crate::W<u32, super::STCR>;
#[doc = "Register STCR `reset()`'s with value 0xc007_5300"]
impl crate::ResetValue for super::STCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc007_5300
    }
}
#[doc = "Reader of field `NOREF`"]
pub type NOREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOREF`"]
pub struct NOREF_W<'a> {
    w: &'a mut W,
}
impl<'a> NOREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SKEW`"]
pub type SKEW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SKEW`"]
pub struct SKEW_W<'a> {
    w: &'a mut W,
}
impl<'a> SKEW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `TENMS`"]
pub type TENMS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `TENMS`"]
pub struct TENMS_W<'a> {
    w: &'a mut W,
}
impl<'a> TENMS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Reads as one. Indicates that no separate reference clock is provided."]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Reads as one. The calibration value is not exactly 10ms because of clock frequency. This could affect its suitability as a software real time clock."]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 0:23 - 23:0\\]
An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. The value read is valid only when core clock is at 48MHz."]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Reads as one. Indicates that no separate reference clock is provided."]
    #[inline(always)]
    pub fn noref(&mut self) -> NOREF_W {
        NOREF_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Reads as one. The calibration value is not exactly 10ms because of clock frequency. This could affect its suitability as a software real time clock."]
    #[inline(always)]
    pub fn skew(&mut self) -> SKEW_W {
        SKEW_W { w: self }
    }
    #[doc = "Bits 24:29 - 29:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. The value read is valid only when core clock is at 48MHz."]
    #[inline(always)]
    pub fn tenms(&mut self) -> TENMS_W {
        TENMS_W { w: self }
    }
}
