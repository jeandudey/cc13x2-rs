#[doc = "Reader of register OSCIMSC"]
pub type R = crate::R<u32, super::OSCIMSC>;
#[doc = "Writer for register OSCIMSC"]
pub type W = crate::W<u32, super::OSCIMSC>;
#[doc = "Register OSCIMSC `reset()`'s with value 0x36"]
impl crate::ResetValue for super::OSCIMSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x36
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HFSRCPENDIM`"]
pub type HFSRCPENDIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFSRCPENDIM`"]
pub struct HFSRCPENDIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HFSRCPENDIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `LFSRCDONEIM`"]
pub type LFSRCDONEIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFSRCDONEIM`"]
pub struct LFSRCDONEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSRCDONEIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `XOSCDLFIM`"]
pub type XOSCDLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCDLFIM`"]
pub struct XOSCDLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCDLFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `XOSCLFIM`"]
pub type XOSCLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCLFIM`"]
pub struct XOSCLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RCOSCDLFIM`"]
pub type RCOSCDLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCDLFIM`"]
pub struct RCOSCDLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCDLFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RCOSCLFIM`"]
pub type RCOSCLFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCLFIM`"]
pub struct RCOSCLFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLFIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `XOSCHFIM`"]
pub type XOSCHFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCHFIM`"]
pub struct XOSCHFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCHFIM_W<'a> {
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
#[doc = "Reader of field `RCOSCHFIM`"]
pub type RCOSCHFIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCHFIM`"]
pub struct RCOSCHFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFIM_W<'a> {
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
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
    #[inline(always)]
    pub fn hfsrcpendim(&self) -> HFSRCPENDIM_R {
        HFSRCPENDIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
    #[inline(always)]
    pub fn lfsrcdoneim(&self) -> LFSRCDONEIM_R {
        LFSRCDONEIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
    #[inline(always)]
    pub fn xoscdlfim(&self) -> XOSCDLFIM_R {
        XOSCDLFIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
    #[inline(always)]
    pub fn xosclfim(&self) -> XOSCLFIM_R {
        XOSCLFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
    #[inline(always)]
    pub fn rcoscdlfim(&self) -> RCOSCDLFIM_R {
        RCOSCDLFIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
    #[inline(always)]
    pub fn rcosclfim(&self) -> RCOSCLFIM_R {
        RCOSCLFIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
    #[inline(always)]
    pub fn xoschfim(&self) -> XOSCHFIM_R {
        XOSCHFIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
    #[inline(always)]
    pub fn rcoschfim(&self) -> RCOSCHFIM_R {
        RCOSCHFIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
    #[inline(always)]
    pub fn hfsrcpendim(&mut self) -> HFSRCPENDIM_W {
        HFSRCPENDIM_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
    #[inline(always)]
    pub fn lfsrcdoneim(&mut self) -> LFSRCDONEIM_W {
        LFSRCDONEIM_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
    #[inline(always)]
    pub fn xoscdlfim(&mut self) -> XOSCDLFIM_W {
        XOSCDLFIM_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
    #[inline(always)]
    pub fn xosclfim(&mut self) -> XOSCLFIM_W {
        XOSCLFIM_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
    #[inline(always)]
    pub fn rcoscdlfim(&mut self) -> RCOSCDLFIM_W {
        RCOSCDLFIM_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
    #[inline(always)]
    pub fn rcosclfim(&mut self) -> RCOSCLFIM_W {
        RCOSCLFIM_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
    #[inline(always)]
    pub fn xoschfim(&mut self) -> XOSCHFIM_W {
        XOSCHFIM_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
    #[inline(always)]
    pub fn rcoschfim(&mut self) -> RCOSCHFIM_W {
        RCOSCHFIM_W { w: self }
    }
}
