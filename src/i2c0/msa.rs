#[doc = "Reader of register MSA"]
pub type R = crate::R<u32, super::MSA>;
#[doc = "Writer for register MSA"]
pub type W = crate::W<u32, super::MSA>;
#[doc = "Register MSA `reset()`'s with value 0"]
impl crate::ResetValue for super::MSA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SA`"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RS_A {
    #[doc = "1: Receive data from slave"]
    RX = 1,
    #[doc = "0: Transmit/send data to slave"]
    TX = 0,
}
impl From<RS_A> for bool {
    #[inline(always)]
    fn from(variant: RS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RS`"]
pub type RS_R = crate::R<bool, RS_A>;
impl RS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RS_A {
        match self.bits {
            true => RS_A::RX,
            false => RS_A::TX,
        }
    }
    #[doc = "Checks if the value of the field is `RX`"]
    #[inline(always)]
    pub fn is_rx(&self) -> bool {
        *self == RS_A::RX
    }
    #[doc = "Checks if the value of the field is `TX`"]
    #[inline(always)]
    pub fn is_tx(&self) -> bool {
        *self == RS_A::TX
    }
}
#[doc = "Write proxy for field `RS`"]
pub struct RS_W<'a> {
    w: &'a mut W,
}
impl<'a> RS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive data from slave"]
    #[inline(always)]
    pub fn rx(self) -> &'a mut W {
        self.variant(RS_A::RX)
    }
    #[doc = "Transmit/send data to slave"]
    #[inline(always)]
    pub fn tx(self) -> &'a mut W {
        self.variant(RS_A::TX)
    }
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
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 1:7 - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 0 - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 1:7 - 7:1\\]
I2C master slave address Defines which slave is addressed for the transaction in master mode"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Bit 0 - 0:0\\]
Receive or Send This bit-field specifies if the next operation is a receive (high) or a transmit/send (low) from the addressed slave SA."]
    #[inline(always)]
    pub fn rs(&mut self) -> RS_W {
        RS_W { w: self }
    }
}
