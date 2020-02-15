#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Writer for register RIS"]
pub type W = crate::W<u32, super::RIS>;
#[doc = "Register RIS `reset()`'s with value 0"]
impl crate::ResetValue for super::RIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Reader of field `WDTRIS`"]
pub type WDTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDTRIS`"]
pub struct WDTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTRIS_W<'a> {
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
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0 - 0:0\\]
This register is the raw interrupt status register. WDT interrupt events can be monitored via this register if the controller interrupt is masked. Value Description 0: The WDT has not timed out 1: A WDT time-out event has occurred"]
    #[inline(always)]
    pub fn wdtris(&self) -> WDTRIS_R {
        WDTRIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
This register is the raw interrupt status register. WDT interrupt events can be monitored via this register if the controller interrupt is masked. Value Description 0: The WDT has not timed out 1: A WDT time-out event has occurred"]
    #[inline(always)]
    pub fn wdtris(&mut self) -> WDTRIS_W {
        WDTRIS_W { w: self }
    }
}
