#[doc = "Reader of register IBRD"]
pub type R = crate::R<u32, super::IBRD>;
#[doc = "Writer for register IBRD"]
pub type W = crate::W<u32, super::IBRD>;
#[doc = "Register IBRD `reset()`'s with value 0"]
impl crate::ResetValue for super::IBRD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DIVINT`"]
pub type DIVINT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVINT`"]
pub struct DIVINT_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    pub fn divint(&self) -> DIVINT_R {
        DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
The integer baud rate divisor: The baud rate divisor is calculated using the formula below: Baud rate divisor = (UART reference clock frequency) / (16 * Baud rate) Baud rate divisor must be minimum 1 and maximum 65535. That is, DIVINT=0 does not give a valid baud rate. Similarly, if DIVINT=0xFFFF, any non-zero values in FBRD.DIVFRAC will be illegal. A valid value must be written to this field before the UART can be used for RX or TX operations."]
    #[inline(always)]
    pub fn divint(&mut self) -> DIVINT_W {
        DIVINT_W { w: self }
    }
}
