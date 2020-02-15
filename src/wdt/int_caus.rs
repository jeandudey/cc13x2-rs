#[doc = "Reader of register INT_CAUS"]
pub type R = crate::R<u32, super::INT_CAUS>;
#[doc = "Writer for register INT_CAUS"]
pub type W = crate::W<u32, super::INT_CAUS>;
#[doc = "Register INT_CAUS `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_CAUS {
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
#[doc = "Reader of field `CAUSE_RESET`"]
pub type CAUSE_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAUSE_RESET`"]
pub struct CAUSE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CAUSE_RESET_W<'a> {
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
#[doc = "Reader of field `CAUSE_INTR`"]
pub type CAUSE_INTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAUSE_INTR`"]
pub struct CAUSE_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CAUSE_INTR_W<'a> {
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
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
    #[inline(always)]
    pub fn cause_reset(&self) -> CAUSE_RESET_R {
        CAUSE_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Replica of RIS.WDTRIS"]
    #[inline(always)]
    pub fn cause_intr(&self) -> CAUSE_INTR_R {
        CAUSE_INTR_R::new((self.bits & 0x01) != 0)
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
Indicates that the cause of an interrupt was a reset generated but blocked due to TEST.TEST_EN (only possible when TEST.TEST_EN is set)."]
    #[inline(always)]
    pub fn cause_reset(&mut self) -> CAUSE_RESET_W {
        CAUSE_RESET_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Replica of RIS.WDTRIS"]
    #[inline(always)]
    pub fn cause_intr(&mut self) -> CAUSE_INTR_W {
        CAUSE_INTR_W { w: self }
    }
}
