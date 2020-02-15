#[doc = "Reader of register RIS"]
pub type R = crate::R<u32, super::RIS>;
#[doc = "Writer for register RIS"]
pub type W = crate::W<u32, super::RIS>;
#[doc = "Register RIS `reset()`'s with value 0x08"]
impl crate::ResetValue for super::RIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x08
    }
}
#[doc = "Reader of field `TXRIS`"]
pub type TXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRIS`"]
pub struct TXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRIS_W<'a> {
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
#[doc = "Reader of field `RXRIS`"]
pub type RXRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRIS`"]
pub struct RXRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRIS_W<'a> {
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
#[doc = "Reader of field `RTRIS`"]
pub type RTRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTRIS`"]
pub struct RTRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTRIS_W<'a> {
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
#[doc = "Reader of field `RORRIS`"]
pub type RORRIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RORRIS`"]
pub struct RORRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RORRIS_W<'a> {
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
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
    #[inline(always)]
    pub fn rorris(&self) -> RORRIS_R {
        RORRIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 3:3\\]
Raw transmit FIFO interrupt status: The transmit interrupt is asserted when there are four or fewer valid entries in the transmit FIFO. The transmit interrupt is not qualified with the SSI enable signal. Therefore one of the following ways can be used: - data can be written to the transmit FIFO prior to enabling the SSI and the interrupts. - SSI and interrupts can be enabled so that data can be written to the transmit FIFO by an interrupt service routine."]
    #[inline(always)]
    pub fn txris(&mut self) -> TXRIS_W {
        TXRIS_W { w: self }
    }
    #[doc = "Bit 2 - 2:2\\]
Raw interrupt state of receive FIFO interrupt: The receive interrupt is asserted when there are four or more valid entries in the receive FIFO."]
    #[inline(always)]
    pub fn rxris(&mut self) -> RXRIS_W {
        RXRIS_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Raw interrupt state of receive timeout interrupt: The receive timeout interrupt is asserted when the receive FIFO is not empty and SSI has remained idle for a fixed 32 bit period. This mechanism can be used to notify the user that data is still present in the receive FIFO and requires servicing. This interrupt is deasserted if the receive FIFO becomes empty by subsequent reads, or if new data is received on RXD. It can also be cleared by writing to ICR.RTIC."]
    #[inline(always)]
    pub fn rtris(&mut self) -> RTRIS_W {
        RTRIS_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt state of receive overrun interrupt: The receive overrun interrupt is asserted when the FIFO is already full and an additional data frame is received, causing an overrun of the FIFO. Data is over-written in the receive shift register, but not the FIFO so the FIFO contents stay valid. It can also be cleared by writing to ICR.RORIC."]
    #[inline(always)]
    pub fn rorris(&mut self) -> RORRIS_W {
        RORRIS_W { w: self }
    }
}
