#[doc = "Reader of register DMAEV"]
pub type R = crate::R<u32, super::DMAEV>;
#[doc = "Writer for register DMAEV"]
pub type W = crate::W<u32, super::DMAEV>;
#[doc = "Register DMAEV `reset()`'s with value 0"]
impl crate::ResetValue for super::DMAEV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | (((value as u32) & 0x000f_ffff) << 12);
        self.w
    }
}
#[doc = "Reader of field `TBMDMAEN`"]
pub type TBMDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMDMAEN`"]
pub struct TBMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CBEDMAEN`"]
pub type CBEDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBEDMAEN`"]
pub struct CBEDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CBMDMAEN`"]
pub type CBMDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMDMAEN`"]
pub struct CBMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `TBTODMAEN`"]
pub type TBTODMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTODMAEN`"]
pub struct TBTODMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTODMAEN_W<'a> {
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
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `TAMDMAEN`"]
pub type TAMDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMDMAEN`"]
pub struct TAMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMDMAEN_W<'a> {
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
#[doc = "Reader of field `RESERVED3`"]
pub type RESERVED3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED3`"]
pub struct RESERVED3_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED3_W<'a> {
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
#[doc = "Reader of field `CAEDMAEN`"]
pub type CAEDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAEDMAEN`"]
pub struct CAEDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEDMAEN_W<'a> {
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
#[doc = "Reader of field `CAMDMAEN`"]
pub type CAMDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMDMAEN`"]
pub struct CAMDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMDMAEN_W<'a> {
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
#[doc = "Reader of field `TATODMAEN`"]
pub type TATODMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATODMAEN`"]
pub struct TATODMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TATODMAEN_W<'a> {
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
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbmdmaen(&self) -> TBMDMAEN_R {
        TBMDMAEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbedmaen(&self) -> CBEDMAEN_R {
        CBEDMAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbmdmaen(&self) -> CBMDMAEN_R {
        CBMDMAEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbtodmaen(&self) -> TBTODMAEN_R {
        TBTODMAEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tamdmaen(&self) -> TAMDMAEN_R {
        TAMDMAEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn caedmaen(&self) -> CAEDMAEN_R {
        CAEDMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn camdmaen(&self) -> CAMDMAEN_R {
        CAMDMAEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tatodmaen(&self) -> TATODMAEN_R {
        TATODMAEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
GPT Timer B Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbmdmaen(&mut self) -> TBMDMAEN_W {
        TBMDMAEN_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
GPT Timer B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbedmaen(&mut self) -> CBEDMAEN_W {
        CBEDMAEN_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
GPT Timer B Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn cbmdmaen(&mut self) -> CBMDMAEN_W {
        CBMDMAEN_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tbtodmaen(&mut self) -> TBTODMAEN_W {
        TBTODMAEN_W { w: self }
    }
    #[doc = "Bits 5:7 - 7:5\\]
Software should not rely on the value of a reserved field. Writing any other value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn tamdmaen(&mut self) -> TAMDMAEN_W {
        TAMDMAEN_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn caedmaen(&mut self) -> CAEDMAEN_W {
        CAEDMAEN_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
GPT Timer A Capture Match DMA Trigger Enable"]
    #[inline(always)]
    pub fn camdmaen(&mut self) -> CAMDMAEN_W {
        CAMDMAEN_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
GPT Timer A Time-Out DMA Trigger Enable"]
    #[inline(always)]
    pub fn tatodmaen(&mut self) -> TATODMAEN_W {
        TATODMAEN_W { w: self }
    }
}
