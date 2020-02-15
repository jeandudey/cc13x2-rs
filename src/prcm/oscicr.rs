#[doc = "Reader of register OSCICR"]
pub type R = crate::R<u32, super::OSCICR>;
#[doc = "Writer for register OSCICR"]
pub type W = crate::W<u32, super::OSCICR>;
#[doc = "Register OSCICR `reset()`'s with value 0"]
impl crate::ResetValue for super::OSCICR {
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
#[doc = "Reader of field `HFSRCPENDC`"]
pub type HFSRCPENDC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFSRCPENDC`"]
pub struct HFSRCPENDC_W<'a> {
    w: &'a mut W,
}
impl<'a> HFSRCPENDC_W<'a> {
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
#[doc = "Reader of field `LFSRCDONEC`"]
pub type LFSRCDONEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LFSRCDONEC`"]
pub struct LFSRCDONEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LFSRCDONEC_W<'a> {
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
#[doc = "Reader of field `XOSCDLFC`"]
pub type XOSCDLFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCDLFC`"]
pub struct XOSCDLFC_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCDLFC_W<'a> {
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
#[doc = "Reader of field `XOSCLFC`"]
pub type XOSCLFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCLFC`"]
pub struct XOSCLFC_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCLFC_W<'a> {
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
#[doc = "Reader of field `RCOSCDLFC`"]
pub type RCOSCDLFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCDLFC`"]
pub struct RCOSCDLFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCDLFC_W<'a> {
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
#[doc = "Reader of field `RCOSCLFC`"]
pub type RCOSCLFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCLFC`"]
pub struct RCOSCLFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCLFC_W<'a> {
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
#[doc = "Reader of field `XOSCHFC`"]
pub type XOSCHFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XOSCHFC`"]
pub struct XOSCHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCHFC_W<'a> {
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
#[doc = "Reader of field `RCOSCHFC`"]
pub type RCOSCHFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSCHFC`"]
pub struct RCOSCHFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSCHFC_W<'a> {
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
Writing 1 to this field clears the HFSRCPEND raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn hfsrcpendc(&self) -> HFSRCPENDC_R {
        HFSRCPENDC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 1 to this field clears the LFSRCDONE raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn lfsrcdonec(&self) -> LFSRCDONEC_R {
        LFSRCDONEC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 1 to this field clears the XOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn xoscdlfc(&self) -> XOSCDLFC_R {
        XOSCDLFC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1 to this field clears the XOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn xosclfc(&self) -> XOSCLFC_R {
        XOSCLFC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 1 to this field clears the RCOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rcoscdlfc(&self) -> RCOSCDLFC_R {
        RCOSCDLFC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 1 to this field clears the RCOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rcosclfc(&self) -> RCOSCLFC_R {
        RCOSCLFC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1 to this field clears the XOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn xoschfc(&self) -> XOSCHFC_R {
        XOSCHFC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 1 to this field clears the RCOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rcoschfc(&self) -> RCOSCHFC_R {
        RCOSCHFC_R::new((self.bits & 0x01) != 0)
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
Writing 1 to this field clears the HFSRCPEND raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn hfsrcpendc(&mut self) -> HFSRCPENDC_W {
        HFSRCPENDC_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 1 to this field clears the LFSRCDONE raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn lfsrcdonec(&mut self) -> LFSRCDONEC_W {
        LFSRCDONEC_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 1 to this field clears the XOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn xoscdlfc(&mut self) -> XOSCDLFC_W {
        XOSCDLFC_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1 to this field clears the XOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn xosclfc(&mut self) -> XOSCLFC_W {
        XOSCLFC_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 1 to this field clears the RCOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rcoscdlfc(&mut self) -> RCOSCDLFC_W {
        RCOSCDLFC_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 1 to this field clears the RCOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rcosclfc(&mut self) -> RCOSCLFC_W {
        RCOSCLFC_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1 to this field clears the XOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn xoschfc(&mut self) -> XOSCHFC_W {
        XOSCHFC_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Writing 1 to this field clears the RCOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rcoschfc(&mut self) -> RCOSCHFC_W {
        RCOSCHFC_W { w: self }
    }
}
