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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RORMIS`"]
pub type RORMIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RORMIS`"]
pub struct RORMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RORMIS_W<'a> {
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
    #[doc = "Bit 3 - 3:3\\]
Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
    #[inline(always)]
    pub fn rormis(&self) -> RORMIS_R {
        RORMIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 3:3\\]
Masked interrupt state of transmit FIFO interrupt: This field returns the masked interrupt state of transmit FIFO interrupt which is the AND product of raw interrupt state RIS.TXRIS and the mask setting IMSC.TXIM."]
    #[inline(always)]
    pub fn txmis(&mut self) -> TXMIS_W {
        TXMIS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Masked interrupt state of receive FIFO interrupt: This field returns the masked interrupt state of receive FIFO interrupt which is the AND product of raw interrupt state RIS.RXRIS and the mask setting IMSC.RXIM."]
    #[inline(always)]
    pub fn rxmis(&mut self) -> RXMIS_W {
        RXMIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Masked interrupt state of receive timeout interrupt: This field returns the masked interrupt state of receive timeout interrupt which is the AND product of raw interrupt state RIS.RTRIS and the mask setting IMSC.RTIM."]
    #[inline(always)]
    pub fn rtmis(&mut self) -> RTMIS_W {
        RTMIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Masked interrupt state of receive overrun interrupt: This field returns the masked interrupt state of receive overrun interrupt which is the AND product of raw interrupt state RIS.RORRIS and the mask setting IMSC.RORIM."]
    #[inline(always)]
    pub fn rormis(&mut self) -> RORMIS_W {
        RORMIS_W { w: self }
    }
}
