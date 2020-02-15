#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0x03"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSY`"]
pub struct BSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BSY_W<'a> {
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
#[doc = "Reader of field `RFF`"]
pub type RFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFF`"]
pub struct RFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RFF_W<'a> {
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
#[doc = "Reader of field `RNE`"]
pub type RNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNE`"]
pub struct RNE_W<'a> {
    w: &'a mut W,
}
impl<'a> RNE_W<'a> {
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
#[doc = "Reader of field `TNF`"]
pub type TNF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TNF`"]
pub struct TNF_W<'a> {
    w: &'a mut W,
}
impl<'a> TNF_W<'a> {
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
#[doc = "Reader of field `TFE`"]
pub type TFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFE`"]
pub struct TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFE_W<'a> {
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
    #[doc = "Bit 4 - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&self) -> RNE_R {
        RNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 4:4\\]
Serial interface busy: 0: SSI is idle 1: SSI is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&mut self) -> BSY_W {
        BSY_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO full: 0: Receive FIFO is not full. 1: Receive FIFO is full."]
    #[inline(always)]
    pub fn rff(&mut self) -> RFF_W {
        RFF_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO not empty 0: Receive FIFO is empty. 1: Receive FIFO is not empty."]
    #[inline(always)]
    pub fn rne(&mut self) -> RNE_W {
        RNE_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full: 0: Transmit FIFO is full. 1: Transmit FIFO is not full."]
    #[inline(always)]
    pub fn tnf(&mut self) -> TNF_W {
        TNF_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty: 0: Transmit FIFO is not empty. 1: Transmit FIFO is empty."]
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W {
        TFE_W { w: self }
    }
}
