#[doc = "Reader of register OSCRIS"]
pub type R = crate::R<u32, super::OSCRIS>;
#[doc = "Writer for register OSCRIS"]
pub type W = crate::W<u32, super::OSCRIS>;
#[doc = "Register OSCRIS `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCRIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `HFSRCPENDRIS`"]
pub type HFSRCPENDRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFSRCPENDRIS`"]
pub struct HFSRCPENDRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFSRCPENDRIS_W<'a> {
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
#[doc = "Reader of field `LFSRCDONERIS`"]
pub type LFSRCDONERIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFSRCDONERIS`"]
pub struct LFSRCDONERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSRCDONERIS_W<'a> {
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
#[doc = "Reader of field `XOSCDLFRIS`"]
pub type XOSCDLFRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCDLFRIS`"]
pub struct XOSCDLFRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCDLFRIS_W<'a> {
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
#[doc = "Reader of field `XOSCLFRIS`"]
pub type XOSCLFRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCLFRIS`"]
pub struct XOSCLFRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLFRIS_W<'a> {
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
#[doc = "Reader of field `RCOSCDLFRIS`"]
pub type RCOSCDLFRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCDLFRIS`"]
pub struct RCOSCDLFRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCDLFRIS_W<'a> {
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
#[doc = "Reader of field `RCOSCLFRIS`"]
pub type RCOSCLFRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCLFRIS`"]
pub struct RCOSCLFRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLFRIS_W<'a> {
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
#[doc = "Reader of field `XOSCHFRIS`"]
pub type XOSCHFRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCHFRIS`"]
pub struct XOSCHFRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCHFRIS_W<'a> {
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
#[doc = "Reader of field `RCOSCHFRIS`"]
pub type RCOSCHFRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCHFRIS`"]
pub struct RCOSCHFRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFRIS_W<'a> {
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
0: HFSRCPEND has not been qualified 1: HFSRCPEND has been qualified since last clear Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
    #[inline(always)]
    pub fn hfsrcpendris(&self) -> HFSRCPENDRIS_R {
        HFSRCPENDRIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: LFSRCDONE has not been qualified 1: LFSRCDONE has been qualified since last clear Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
    #[inline(always)]
    pub fn lfsrcdoneris(&self) -> LFSRCDONERIS_R {
        LFSRCDONERIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
    #[inline(always)]
    pub fn xoscdlfris(&self) -> XOSCDLFRIS_R {
        XOSCDLFRIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: XOSCLF has not been qualified 1: XOSCLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
    #[inline(always)]
    pub fn xosclfris(&self) -> XOSCLFRIS_R {
        XOSCLFRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
    #[inline(always)]
    pub fn rcoscdlfris(&self) -> RCOSCDLFRIS_R {
        RCOSCDLFRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
    #[inline(always)]
    pub fn rcosclfris(&self) -> RCOSCLFRIS_R {
        RCOSCLFRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: XOSCHF has not been qualified 1: XOSCHF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
    #[inline(always)]
    pub fn xoschfris(&self) -> XOSCHFRIS_R {
        XOSCHFRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: RCOSCHF has not been qualified 1: RCOSCHF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
    #[inline(always)]
    pub fn rcoschfris(&self) -> RCOSCHFRIS_R {
        RCOSCHFRIS_R::new((self.bits & 0x01) != 0)
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
0: HFSRCPEND has not been qualified 1: HFSRCPEND has been qualified since last clear Interrupt is qualified regardless of OSCIMSC.HFSRCPENDIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.HFSRCPENDC"]
    #[inline(always)]
    pub fn hfsrcpendris(&mut self) -> HFSRCPENDRIS_W {
        HFSRCPENDRIS_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
0: LFSRCDONE has not been qualified 1: LFSRCDONE has been qualified since last clear Interrupt is qualified regardless of OSCIMSC.LFSRCDONEIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.LFSRCDONEC"]
    #[inline(always)]
    pub fn lfsrcdoneris(&mut self) -> LFSRCDONERIS_W {
        LFSRCDONERIS_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: XOSCDLF has not been qualified 1: XOSCDLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCDLFC"]
    #[inline(always)]
    pub fn xoscdlfris(&mut self) -> XOSCDLFRIS_W {
        XOSCDLFRIS_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: XOSCLF has not been qualified 1: XOSCLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCLFC"]
    #[inline(always)]
    pub fn xosclfris(&mut self) -> XOSCLFRIS_W {
        XOSCLFRIS_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
0: RCOSCDLF has not been qualified 1: RCOSCDLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCDLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCDLFC"]
    #[inline(always)]
    pub fn rcoscdlfris(&mut self) -> RCOSCDLFRIS_W {
        RCOSCDLFRIS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: RCOSCLF has not been qualified 1: RCOSCLF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCLFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCLFC"]
    #[inline(always)]
    pub fn rcosclfris(&mut self) -> RCOSCLFRIS_W {
        RCOSCLFRIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: XOSCHF has not been qualified 1: XOSCHF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.XOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.XOSCHFC"]
    #[inline(always)]
    pub fn xoschfris(&mut self) -> XOSCHFRIS_W {
        XOSCHFRIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: RCOSCHF has not been qualified 1: RCOSCHF has been qualified since last clear. Interrupt is qualified regardless of OSCIMSC.RCOSCHFIM setting. The order of qualifying raw interrupt and enable of interrupt mask is indifferent for generating an OSC Interrupt. Set by HW. Cleared by writing to OSCICR.RCOSCHFC"]
    #[inline(always)]
    pub fn rcoschfris(&mut self) -> RCOSCHFRIS_W {
        RCOSCHFRIS_W { w: self }
    }
}
