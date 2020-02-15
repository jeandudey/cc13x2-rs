#[doc = "Reader of register DR"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Writer for register DR"]
pub type W = crate::W<u32, super::DR>;
#[doc = "Register DR `reset()`'s with value 0"]
impl crate::ResetValue for super::DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OE`"]
pub type OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OE`"]
pub struct OE_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_W<'a> {
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
#[doc = "Reader of field `BE`"]
pub type BE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BE`"]
pub struct BE_W<'a> {
    w: &'a mut W,
}
impl<'a> BE_W<'a> {
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
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
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
#[doc = "Reader of field `FE`"]
pub type FE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FE`"]
pub struct FE_W<'a> {
    w: &'a mut W,
}
impl<'a> FE_W<'a> {
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
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - 11:11\\]
UART Overrun Error: This bit is set to 1 if data is received and the receive FIFO is already full. The FIFO contents remain valid because no more data is written when the FIFO is full, , only the contents of the shift register are overwritten. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
UART Break Error: This bit is set to 1 if a break condition was detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO (that is., the oldest received data character since last read). When a break occurs, a 0 character is loaded into the FIFO. The next character is enabled after the receive data input (UARTRXD input pin) goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
UART Parity Error: When set to 1, it indicates that the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select. In FIFO mode, this error is associated with the character at the top of the FIFO (that is, the oldest received data character since last read)."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
UART Framing Error: When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO (that is., the oldest received data character since last read)."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Data transmitted or received: On writes, the transmit data character is pushed into the FIFO. On reads, the oldest received data character since the last read is returned."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - 11:11\\]
UART Overrun Error: This bit is set to 1 if data is received and the receive FIFO is already full. The FIFO contents remain valid because no more data is written when the FIFO is full, , only the contents of the shift register are overwritten. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    pub fn oe(&mut self) -> OE_W {
        OE_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
UART Break Error: This bit is set to 1 if a break condition was detected, indicating that the received data input (UARTRXD input pin) was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO (that is., the oldest received data character since last read). When a break occurs, a 0 character is loaded into the FIFO. The next character is enabled after the receive data input (UARTRXD input pin) goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub fn be(&mut self) -> BE_W {
        BE_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
UART Parity Error: When set to 1, it indicates that the parity of the received data character does not match the parity that the LCRH.EPS and LCRH.SPS select. In FIFO mode, this error is associated with the character at the top of the FIFO (that is, the oldest received data character since last read)."]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
UART Framing Error: When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO (that is., the oldest received data character since last read)."]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W {
        FE_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Data transmitted or received: On writes, the transmit data character is pushed into the FIFO. On reads, the oldest received data character since the last read is returned."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
