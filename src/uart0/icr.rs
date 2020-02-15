#[doc = "Reader of register ICR"]
pub type R = crate::R<u32, super::ICR>;
#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
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
#[doc = "Reader of field `EOTIC`"]
pub type EOTIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOTIC`"]
pub struct EOTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTIC_W<'a> {
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
#[doc = "Reader of field `OEIC`"]
pub type OEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OEIC`"]
pub struct OEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIC_W<'a> {
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
#[doc = "Reader of field `BEIC`"]
pub type BEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BEIC`"]
pub struct BEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIC_W<'a> {
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
#[doc = "Reader of field `PEIC`"]
pub type PEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEIC`"]
pub struct PEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIC_W<'a> {
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
#[doc = "Reader of field `FEIC`"]
pub type FEIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FEIC`"]
pub struct FEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIC_W<'a> {
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
#[doc = "Reader of field `RTIC`"]
pub type RTIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTIC`"]
pub struct RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIC_W<'a> {
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
#[doc = "Reader of field `TXIC`"]
pub type TXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXIC`"]
pub struct TXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIC_W<'a> {
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
#[doc = "Reader of field `RXIC`"]
pub type RXIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXIC`"]
pub struct RXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIC_W<'a> {
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
#[doc = "Reader of field `CTSMIC`"]
pub type CTSMIC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSMIC`"]
pub struct CTSMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIC_W<'a> {
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
End of Transmission interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.EOTRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn eotic(&self) -> EOTIC_R {
        EOTIC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.OERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn oeic(&self) -> OEIC_R {
        OEIC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt clear: Writing 1 to this field clears the break error interrupt (RIS.BERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn beic(&self) -> BEIC_R {
        BEIC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt clear: Writing 1 to this field clears the parity error interrupt (RIS.PERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn peic(&self) -> PEIC_R {
        PEIC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt clear: Writing 1 to this field clears the framing error interrupt (RIS.FERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn feic(&self) -> FEIC_R {
        FEIC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt clear: Writing 1 to this field clears the receive timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt clear: Writing 1 to this field clears the transmit interrupt (RIS.TXRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn txic(&self) -> TXIC_R {
        TXIC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt clear: Writing 1 to this field clears the receive interrupt (RIS.RXRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn rxic(&self) -> RXIC_R {
        RXIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0"]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt clear: Writing 1 to this field clears the clear to send interrupt (RIS.CTSRMIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn ctsmic(&self) -> CTSMIC_R {
        CTSMIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0."]
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
End of Transmission interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.EOTRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn eotic(&mut self) -> EOTIC_W {
        EOTIC_W { w: self }
    }
    #[doc = "Bit 10 - 10:10\\]
Overrun error interrupt clear: Writing 1 to this field clears the overrun error interrupt (RIS.OERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn oeic(&mut self) -> OEIC_W {
        OEIC_W { w: self }
    }
    #[doc = "Bit 9 - 9:9\\]
Break error interrupt clear: Writing 1 to this field clears the break error interrupt (RIS.BERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn beic(&mut self) -> BEIC_W {
        BEIC_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Parity error interrupt clear: Writing 1 to this field clears the parity error interrupt (RIS.PERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn peic(&mut self) -> PEIC_W {
        PEIC_W { w: self }
    }
    #[doc = "Bit 7 - 7:7\\]
Framing error interrupt clear: Writing 1 to this field clears the framing error interrupt (RIS.FERIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn feic(&mut self) -> FEIC_W {
        FEIC_W { w: self }
    }
    #[doc = "Bit 6 - 6:6\\]
Receive timeout interrupt clear: Writing 1 to this field clears the receive timeout interrupt (RIS.RTRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W {
        RTIC_W { w: self }
    }
    #[doc = "Bit 5 - 5:5\\]
Transmit interrupt clear: Writing 1 to this field clears the transmit interrupt (RIS.TXRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W {
        TXIC_W { w: self }
    }
    #[doc = "Bit 4 - 4:4\\]
Receive interrupt clear: Writing 1 to this field clears the receive interrupt (RIS.RXRIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W {
        RXIC_W { w: self }
    }
    #[doc = "Bits 2:3 - 3:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> RESERVED2_W {
        RESERVED2_W { w: self }
    }
    #[doc = "Bit 1 - 1:1\\]
Clear to Send (CTS) modem interrupt clear: Writing 1 to this field clears the clear to send interrupt (RIS.CTSRMIS). Writing 0 has no effect."]
    #[inline(always)]
    pub fn ctsmic(&mut self) -> CTSMIC_W {
        CTSMIC_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior. Write 0."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
