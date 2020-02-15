#[doc = "Reader of register ICLR"]
pub type R = crate::R<u32, super::ICLR>;
#[doc = "Writer for register ICLR"]
pub type W = crate::W<u32, super::ICLR>;
#[doc = "Register ICLR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED14`"]
pub type RESERVED14_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED14`"]
pub struct RESERVED14_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 14)) | (((value as u32) & 0x0003_ffff) << 14);
        self.w
    }
}
#[doc = "Reader of field `DMABINT`"]
pub type DMABINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMABINT`"]
pub struct DMABINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABINT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RESERVED12`"]
pub type RESERVED12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED12`"]
pub struct RESERVED12_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TBMCINT`"]
pub type TBMCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMCINT`"]
pub struct TBMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMCINT_W<'a> {
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
#[doc = "Reader of field `CBECINT`"]
pub type CBECINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBECINT`"]
pub struct CBECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBECINT_W<'a> {
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
#[doc = "Reader of field `CBMCINT`"]
pub type CBMCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMCINT`"]
pub struct CBMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMCINT_W<'a> {
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
#[doc = "Reader of field `TBTOCINT`"]
pub type TBTOCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTOCINT`"]
pub struct TBTOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOCINT_W<'a> {
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
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMAAINT`"]
pub type DMAAINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAAINT`"]
pub struct DMAAINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAAINT_W<'a> {
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
#[doc = "Reader of field `TAMCINT`"]
pub type TAMCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMCINT`"]
pub struct TAMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMCINT_W<'a> {
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
#[doc = "Reader of field `CAECINT`"]
pub type CAECINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAECINT`"]
pub struct CAECINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAECINT_W<'a> {
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
#[doc = "Reader of field `CAMCINT`"]
pub type CAMCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMCINT`"]
pub struct CAMCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMCINT_W<'a> {
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
#[doc = "Reader of field `TATOCINT`"]
pub type TATOCINT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATOCINT`"]
pub struct TATOCINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOCINT_W<'a> {
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
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new(((self.bits >> 14) & 0x0003_ffff) as u32)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
    #[inline(always)]
    pub fn dmabint(&self) -> DMABINT_R {
        DMABINT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
    #[inline(always)]
    pub fn tbmcint(&self) -> TBMCINT_R {
        TBMCINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
    #[inline(always)]
    pub fn cbecint(&self) -> CBECINT_R {
        CBECINT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
    #[inline(always)]
    pub fn cbmcint(&self) -> CBMCINT_R {
        CBMCINT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
    #[inline(always)]
    pub fn tbtocint(&self) -> TBTOCINT_R {
        TBTOCINT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
    #[inline(always)]
    pub fn dmaaint(&self) -> DMAAINT_R {
        DMAAINT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
    #[inline(always)]
    pub fn tamcint(&self) -> TAMCINT_R {
        TAMCINT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
    #[inline(always)]
    pub fn caecint(&self) -> CAECINT_R {
        CAECINT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
    #[inline(always)]
    pub fn camcint(&self) -> CAMCINT_R {
        CAMCINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
    #[inline(always)]
    pub fn tatocint(&self) -> TATOCINT_R {
        TATOCINT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&mut self) -> RESERVED14_W {
        RESERVED14_W { w: self }
    }
    #[doc = "Bit 13 - 13:13\\]
0: Do nothing. 1: Clear RIS.DMABRIS and MIS.DMABMIS"]
    #[inline(always)]
    pub fn dmabint(&mut self) -> DMABINT_W {
        DMABINT_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
0: Do nothing. 1: Clear RIS.TBMRIS and MIS.TBMMIS"]
    #[inline(always)]
    pub fn tbmcint(&mut self) -> TBMCINT_W {
        TBMCINT_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
0: Do nothing. 1: Clear RIS.CBERIS and MIS.CBEMIS"]
    #[inline(always)]
    pub fn cbecint(&mut self) -> CBECINT_W {
        CBECINT_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
0: Do nothing. 1: Clear RIS.CBMRIS and MIS.CBMMIS"]
    #[inline(always)]
    pub fn cbmcint(&mut self) -> CBMCINT_W {
        CBMCINT_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: Do nothing. 1: Clear RIS.TBTORIS and MIS.TBTOMIS"]
    #[inline(always)]
    pub fn tbtocint(&mut self) -> TBTOCINT_W {
        TBTOCINT_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: Do nothing. 1: Clear RIS.DMAARIS and MIS.DMAAMIS"]
    #[inline(always)]
    pub fn dmaaint(&mut self) -> DMAAINT_W {
        DMAAINT_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: Do nothing. 1: Clear RIS.TAMRIS and MIS.TAMMIS"]
    #[inline(always)]
    pub fn tamcint(&mut self) -> TAMCINT_W {
        TAMCINT_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: Do nothing. 1: Clear RIS.CAERIS and MIS.CAEMIS"]
    #[inline(always)]
    pub fn caecint(&mut self) -> CAECINT_W {
        CAECINT_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: Do nothing. 1: Clear RIS.CAMRIS and MIS.CAMMIS"]
    #[inline(always)]
    pub fn camcint(&mut self) -> CAMCINT_W {
        CAMCINT_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: Do nothing. 1: Clear RIS.TATORIS and MIS.TATOMIS"]
    #[inline(always)]
    pub fn tatocint(&mut self) -> TATOCINT_W {
        TATOCINT_W { w: self }
    }
}
