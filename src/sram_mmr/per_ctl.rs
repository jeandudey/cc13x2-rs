#[doc = "Reader of register PER_CTL"]
pub type R = crate::R<u32, super::PER_CTL>;
#[doc = "Writer for register PER_CTL"]
pub type W = crate::W<u32, super::PER_CTL>;
#[doc = "Register PER_CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PER_CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED9`"]
pub type RESERVED9_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED9`"]
pub struct RESERVED9_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
#[doc = "Reader of field `PER_DISABLE`"]
pub type PER_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER_DISABLE`"]
pub struct PER_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_DISABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Reader of field `PER_DEBUG_ENABLE`"]
pub type PER_DEBUG_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PER_DEBUG_ENABLE`"]
pub struct PER_DEBUG_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_DEBUG_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
    #[inline(always)]
    pub fn per_disable(&self) -> PER_DISABLE_R {
        PER_DISABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
    #[inline(always)]
    pub fn per_debug_enable(&self) -> PER_DEBUG_ENABLE_R {
        PER_DEBUG_ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&mut self) -> RESERVED9_W {
        RESERVED9_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Parity Status Disable 0: A parity error will update PER_CHK.PER_ADDR field 1: Parity error does not update PER_CHK.PER_ADDR field"]
    #[inline(always)]
    pub fn per_disable(&mut self) -> PER_DISABLE_W {
        PER_DISABLE_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Parity Error Debug Enable 0: Normal operation 1: An address offset can be written to PER_DBG.PER_DEBUG_ADDR and parity errors will be generated on reads from within this offset"]
    #[inline(always)]
    pub fn per_debug_enable(&mut self) -> PER_DEBUG_ENABLE_W {
        PER_DEBUG_ENABLE_W { w: self }
    }
}
