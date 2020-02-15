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
#[doc = "Reader of field `EOTMIS`"]
pub type EOTMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOTMIS`"]
pub struct EOTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTMIS_W<'a> {
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
#[doc = "Reader of field `OEMIS`"]
pub type OEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEMIS`"]
pub struct OEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OEMIS_W<'a> {
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
#[doc = "Reader of field `BEMIS`"]
pub type BEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEMIS`"]
pub struct BEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BEMIS_W<'a> {
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
#[doc = "Reader of field `PEMIS`"]
pub type PEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEMIS`"]
pub struct PEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PEMIS_W<'a> {
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
#[doc = "Reader of field `FEMIS`"]
pub type FEMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEMIS`"]
pub struct FEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FEMIS_W<'a> {
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
#[doc = "Reader of field `RTMIS`"]
pub type RTMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTMIS`"]
pub struct RTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMIS_W<'a> {
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
#[doc = "Reader of field `TXMIS`"]
pub type TXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMIS`"]
pub struct TXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIS_W<'a> {
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
#[doc = "Reader of field `RXMIS`"]
pub type RXMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMIS`"]
pub struct RXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMIS_W<'a> {
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
#[doc = "Reader of field `RESERVED2`"]
pub type RESERVED2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED2`"]
pub struct RESERVED2_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `CTSMMIS`"]
pub type CTSMMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSMMIS`"]
pub struct CTSMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMMIS_W<'a> {
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
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 11 - 11:11\\]
End of Transmission interrupt status: This field returns the masked interrupt state of the overrun interrupt which is the AND product of raw interrupt state RIS.EOTRIS and the mask setting IMSC.EOTIM."]
    #[inline(always)]
    pub fn eotmis(&self) -> EOTMIS_R {
        EOTMIS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error masked interrupt status: This field returns the masked interrupt state of the overrun interrupt which is the AND product of raw interrupt state RIS.OERIS and the mask setting IMSC.OEIM."]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error masked interrupt status: This field returns the masked interrupt state of the break error interrupt which is the AND product of raw interrupt state RIS.BERIS and the mask setting IMSC.BEIM."]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error masked interrupt status: This field returns the masked interrupt state of the parity error interrupt which is the AND product of raw interrupt state RIS.PERIS and the mask setting IMSC.PEIM."]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error masked interrupt status: Returns the masked interrupt state of the framing error interrupt which is the AND product of raw interrupt state RIS.FERIS and the mask setting IMSC.FEIM."]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout masked interrupt status: Returns the masked interrupt state of the receive timeout interrupt. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from RTMIS and RIS.RTRIS."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit masked interrupt status: This field returns the masked interrupt state of the transmit interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive masked interrupt status: This field returns the masked interrupt state of the receive interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem masked interrupt status: This field returns the masked interrupt state of the clear to send interrupt which is the AND product of raw interrupt state RIS.CTSRMIS and the mask setting IMSC.CTSMIM."]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CTSMMIS_R {
        CTSMMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&mut self) -> RESERVED12_W {
        RESERVED12_W { w: self }
    }
    #[doc = "Bit 11 - 11:11\\]
End of Transmission interrupt status: This field returns the masked interrupt state of the overrun interrupt which is the AND product of raw interrupt state RIS.EOTRIS and the mask setting IMSC.EOTIM."]
    #[inline(always)]
    pub fn eotmis(&mut self) -> EOTMIS_W {
        EOTMIS_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error masked interrupt status: This field returns the masked interrupt state of the overrun interrupt which is the AND product of raw interrupt state RIS.OERIS and the mask setting IMSC.OEIM."]
    #[inline(always)]
    pub fn oemis(&mut self) -> OEMIS_W {
        OEMIS_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Break error masked interrupt status: This field returns the masked interrupt state of the break error interrupt which is the AND product of raw interrupt state RIS.BERIS and the mask setting IMSC.BEIM."]
    #[inline(always)]
    pub fn bemis(&mut self) -> BEMIS_W {
        BEMIS_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error masked interrupt status: This field returns the masked interrupt state of the parity error interrupt which is the AND product of raw interrupt state RIS.PERIS and the mask setting IMSC.PEIM."]
    #[inline(always)]
    pub fn pemis(&mut self) -> PEMIS_W {
        PEMIS_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error masked interrupt status: Returns the masked interrupt state of the framing error interrupt which is the AND product of raw interrupt state RIS.FERIS and the mask setting IMSC.FEIM."]
    #[inline(always)]
    pub fn femis(&mut self) -> FEMIS_W {
        FEMIS_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout masked interrupt status: Returns the masked interrupt state of the receive timeout interrupt. The raw interrupt for receive timeout cannot be set unless the mask is set (IMSC.RTIM = 1). This is because the mask acts as an enable for power saving. That is, the same status can be read from RTMIS and RIS.RTRIS."]
    #[inline(always)]
    pub fn rtmis(&mut self) -> RTMIS_W {
        RTMIS_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit masked interrupt status: This field returns the masked interrupt state of the transmit interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[inline(always)]
    pub fn txmis(&mut self) -> TXMIS_W {
        TXMIS_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Receive masked interrupt status: This field returns the masked interrupt state of the receive interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[inline(always)]
    pub fn rxmis(&mut self) -> RXMIS_W {
        RXMIS_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem masked interrupt status: This field returns the masked interrupt state of the clear to send interrupt which is the AND product of raw interrupt state RIS.CTSRMIS and the mask setting IMSC.CTSMIM."]
    #[inline(always)]
    pub fn ctsmmis(&mut self) -> CTSMMIS_W {
        CTSMMIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
