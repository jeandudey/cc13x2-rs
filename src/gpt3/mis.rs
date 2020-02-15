#[doc = "Reader of register MIS"]
pub type R = crate::R<u32, super::MIS>;
#[doc = "Writer for register MIS"]
pub type W = crate::W<u32, super::MIS>;
#[doc = "Register MIS `reset()`'s with value 0"]
impl crate::ResetValue for super::MIS {
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
#[doc = "Reader of field `DMABMIS`"]
pub type DMABMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMABMIS`"]
pub struct DMABMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMABMIS_W<'a> {
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
#[doc = "Reader of field `TBMMIS`"]
pub type TBMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBMMIS`"]
pub struct TBMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBMMIS_W<'a> {
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
#[doc = "Reader of field `CBEMIS`"]
pub type CBEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBEMIS`"]
pub struct CBEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CBEMIS_W<'a> {
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
#[doc = "Reader of field `CBMMIS`"]
pub type CBMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBMMIS`"]
pub struct CBMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CBMMIS_W<'a> {
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
#[doc = "Reader of field `TBTOMIS`"]
pub type TBTOMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TBTOMIS`"]
pub struct TBTOMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TBTOMIS_W<'a> {
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
#[doc = "Reader of field `DMAAMIS`"]
pub type DMAAMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAAMIS`"]
pub struct DMAAMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAAMIS_W<'a> {
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
#[doc = "Reader of field `TAMMIS`"]
pub type TAMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAMMIS`"]
pub struct TAMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMMIS_W<'a> {
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
#[doc = "Reader of field `CAEMIS`"]
pub type CAEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAEMIS`"]
pub struct CAEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAEMIS_W<'a> {
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
#[doc = "Reader of field `CAMMIS`"]
pub type CAMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAMMIS`"]
pub struct CAMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CAMMIS_W<'a> {
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
#[doc = "Reader of field `TATOMIS`"]
pub type TATOMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TATOMIS`"]
pub struct TATOMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TATOMIS_W<'a> {
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
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 && IMR.DMABIM = 1"]
    #[inline(always)]
    pub fn dmabmis(&self) -> DMABMIS_R {
        DMABMIS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 && IMR.TBMIM = 1"]
    #[inline(always)]
    pub fn tbmmis(&self) -> TBMMIS_R {
        TBMMIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 && IMR.CBEIM = 1"]
    #[inline(always)]
    pub fn cbemis(&self) -> CBEMIS_R {
        CBEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 && IMR.CBMIM = 1"]
    #[inline(always)]
    pub fn cbmmis(&self) -> CBMMIS_R {
        CBMMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 && IMR.TBTOIM = 1"]
    #[inline(always)]
    pub fn tbtomis(&self) -> TBTOMIS_R {
        TBTOMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 && IMR.DMAAIM = 1"]
    #[inline(always)]
    pub fn dmaamis(&self) -> DMAAMIS_R {
        DMAAMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 && IMR.TAMIM = 1"]
    #[inline(always)]
    pub fn tammis(&self) -> TAMMIS_R {
        TAMMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 && IMR.CAEIM = 1"]
    #[inline(always)]
    pub fn caemis(&self) -> CAEMIS_R {
        CAEMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 && IMR.CAMIM = 1"]
    #[inline(always)]
    pub fn cammis(&self) -> CAMMIS_R {
        CAMMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 && IMR.TATOIM = 1"]
    #[inline(always)]
    pub fn tatomis(&self) -> TATOMIS_R {
        TATOMIS_R::new((self.bits & 0x01) != 0)
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
0: No interrupt or interrupt not enabled 1: RIS.DMABRIS = 1 && IMR.DMABIM = 1"]
    #[inline(always)]
    pub fn dmabmis(&mut self) -> DMABMIS_W {
        DMABMIS_W { w: self }
    }
    #[doc = "Bit 12 - 12:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
0: No interrupt or interrupt not enabled 1: RIS.TBMRIS = 1 && IMR.TBMIM = 1"]
    #[inline(always)]
    pub fn tbmmis(&mut self) -> TBMMIS_W {
        TBMMIS_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
0: No interrupt or interrupt not enabled 1: RIS.CBERIS = 1 && IMR.CBEIM = 1"]
    #[inline(always)]
    pub fn cbemis(&mut self) -> CBEMIS_W {
        CBEMIS_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
0: No interrupt or interrupt not enabled 1: RIS.CBMRIS = 1 && IMR.CBMIM = 1"]
    #[inline(always)]
    pub fn cbmmis(&mut self) -> CBMMIS_W {
        CBMMIS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
0: No interrupt or interrupt not enabled 1: RIS.TBTORIS = 1 && IMR.TBTOIM = 1"]
    #[inline(always)]
    pub fn tbtomis(&mut self) -> TBTOMIS_W {
        TBTOMIS_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
0: No interrupt or interrupt not enabled 1: RIS.DMAARIS = 1 && IMR.DMAAIM = 1"]
    #[inline(always)]
    pub fn dmaamis(&mut self) -> DMAAMIS_W {
        DMAAMIS_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
0: No interrupt or interrupt not enabled 1: RIS.TAMRIS = 1 && IMR.TAMIM = 1"]
    #[inline(always)]
    pub fn tammis(&mut self) -> TAMMIS_W {
        TAMMIS_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&mut self) -> RESERVED3_W {
        RESERVED3_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
0: No interrupt or interrupt not enabled 1: RIS.CAERIS = 1 && IMR.CAEIM = 1"]
    #[inline(always)]
    pub fn caemis(&mut self) -> CAEMIS_W {
        CAEMIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
0: No interrupt or interrupt not enabled 1: RIS.CAMRIS = 1 && IMR.CAMIM = 1"]
    #[inline(always)]
    pub fn cammis(&mut self) -> CAMMIS_W {
        CAMMIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
0: No interrupt or interrupt not enabled 1: RIS.TATORIS = 1 && IMR.TATOIM = 1"]
    #[inline(always)]
    pub fn tatomis(&mut self) -> TATOMIS_W {
        TATOMIS_W { w: self }
    }
}
