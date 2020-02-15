#[doc = "Reader of register FR"]
pub type R = crate::R<u32, super::FR>;
#[doc = "Writer for register FR"]
pub type W = crate::W<u32, super::FR>;
#[doc = "Register FR `reset()`'s with value 0x90"]
impl crate::ResetValue for super::FR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x90
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXFE`"]
pub type TXFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFE`"]
pub struct TXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFE_W<'a> {
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
#[doc = "Reader of field `RXFF`"]
pub type RXFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFF`"]
pub struct RXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFF_W<'a> {
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
#[doc = "Reader of field `TXFF`"]
pub type TXFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFF`"]
pub struct TXFF_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFF_W<'a> {
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
#[doc = "Reader of field `RXFE`"]
pub type RXFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXFE`"]
pub struct RXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFE_W<'a> {
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
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUSY`"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `CTS`"]
pub type CTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTS`"]
pub struct CTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_W<'a> {
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
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Transmit FIFO Empty: The meaning of this bit depends on the state of LCRH.FEN . - If the FIFO is disabled, this bit is set when the transmit holding register is empty. - If the FIFO is enabled, this bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
UART Receive FIFO Full: The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the receive holding register is full. - If the FIFO is enabled, this bit is set when the receive FIFO is full."]
    #[inline(always)]
    pub fn rxff(&self) -> RXFF_R {
        RXFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
UART Transmit FIFO Full: Transmit FIFO full. The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the transmit holding register is full. - If the FIFO is enabled, this bit is set when the transmit FIFO is full."]
    #[inline(always)]
    pub fn txff(&self) -> TXFF_R {
        TXFF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
UART Receive FIFO Empty: Receive FIFO empty. The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the receive holding register is empty. - If the FIFO is enabled, this bit is set when the receive FIFO is empty."]
    #[inline(always)]
    pub fn rxfe(&self) -> RXFE_R {
        RXFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
UART Busy: If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Clear To Send: This bit is the complement of the active-low UART CTS input pin. That is, the bit is 1 when CTS input pin is LOW."]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
UART Transmit FIFO Empty: The meaning of this bit depends on the state of LCRH.FEN . - If the FIFO is disabled, this bit is set when the transmit holding register is empty. - If the FIFO is enabled, this bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
    #[inline(always)]
    pub fn txfe(&mut self) -> TXFE_W {
        TXFE_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
UART Receive FIFO Full: The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the receive holding register is full. - If the FIFO is enabled, this bit is set when the receive FIFO is full."]
    #[inline(always)]
    pub fn rxff(&mut self) -> RXFF_W {
        RXFF_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
UART Transmit FIFO Full: Transmit FIFO full. The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the transmit holding register is full. - If the FIFO is enabled, this bit is set when the transmit FIFO is full."]
    #[inline(always)]
    pub fn txff(&mut self) -> TXFF_W {
        TXFF_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
UART Receive FIFO Empty: Receive FIFO empty. The meaning of this bit depends on the state of LCRH.FEN. - If the FIFO is disabled, this bit is set when the receive holding register is empty. - If the FIFO is enabled, this bit is set when the receive FIFO is empty."]
    #[inline(always)]
    pub fn rxfe(&mut self) -> RXFE_W {
        RXFE_W { w: self }
    }
    #[doc = "Bit 3 - 3:3\\]
UART Busy: If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bits 1:2 - 2:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Clear To Send: This bit is the complement of the active-low UART CTS input pin. That is, the bit is 1 when CTS input pin is LOW."]
    #[inline(always)]
    pub fn cts(&mut self) -> CTS_W {
        CTS_W { w: self }
    }
}
