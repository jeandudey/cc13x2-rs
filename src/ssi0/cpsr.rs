#[doc = "Reader of register CPSR"]
pub type R = crate::R<u32, super::CPSR>;
#[doc = "Writer for register CPSR"]
pub type W = crate::W<u32, super::CPSR>;
#[doc = "Register CPSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CPSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CPSDVSR`"]
pub type CPSDVSR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CPSDVSR`"]
pub struct CPSDVSR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPSDVSR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CPSDVSR_R {
        CPSDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Clock prescale divisor: This field specifies the division factor by which the input system clock to SSI must be internally divided before further use. The value programmed into this field must be an even non-zero number (2-254). The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero."]
    #[inline(always)]
    pub fn cpsdvsr(&mut self) -> CPSDVSR_W {
        CPSDVSR_W { w: self }
    }
}
