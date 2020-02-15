#[doc = "Reader of register DACSTAT"]
pub type R = crate::R<u32, super::DACSTAT>;
#[doc = "Writer for register DACSTAT"]
pub type W = crate::W<u32, super::DACSTAT>;
#[doc = "Register DACSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::DACSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `SETUP_ACTIVE`"]
pub type SETUP_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SETUP_ACTIVE`"]
pub struct SETUP_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_ACTIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `HOLD_ACTIVE`"]
pub type HOLD_ACTIVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOLD_ACTIVE`"]
pub struct HOLD_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOLD_ACTIVE_W<'a> {
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
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
    #[doc = "Bit 1 - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
    #[inline(always)]
    pub fn setup_active(&self) -> SETUP_ACTIVE_R {
        SETUP_ACTIVE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
    #[inline(always)]
    pub fn hold_active(&self) -> HOLD_ACTIVE_R {
        HOLD_ACTIVE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
DAC setup phase status. 0: Sample clock is disabled or setup phase is complete. 1: Setup phase in progress."]
    #[inline(always)]
    pub fn setup_active(&mut self) -> SETUP_ACTIVE_W {
        SETUP_ACTIVE_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
DAC hold phase status. 0: Sample clock is disabled or DAC is not in hold phase. 1: Hold phase in progress."]
    #[inline(always)]
    pub fn hold_active(&mut self) -> HOLD_ACTIVE_W {
        HOLD_ACTIVE_W { w: self }
    }
}
