#[doc = "Reader of register ID_MMFR2"]
pub type R = crate::R<u32, super::ID_MMFR2>;
#[doc = "Writer for register ID_MMFR2"]
pub type W = crate::W<u32, super::ID_MMFR2>;
#[doc = "Register ID_MMFR2 `reset()`'s with value 0x0100_0000"]
impl crate::ResetValue for super::ID_MMFR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0000
    }
}
#[doc = "Reader of field `RESERVED28`"]
pub type RESERVED28_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED28`"]
pub struct RESERVED28_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 25)) | (((value as u32) & 0x7f) << 25);
        self.w
    }
}
#[doc = "Reader of field `WAIT_FOR_INTERRUPT_STALLING`"]
pub type WAIT_FOR_INTERRUPT_STALLING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAIT_FOR_INTERRUPT_STALLING`"]
pub struct WAIT_FOR_INTERRUPT_STALLING_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_FOR_INTERRUPT_STALLING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
    #[inline(always)]
    pub fn wait_for_interrupt_stalling(&self) -> WAIT_FOR_INTERRUPT_STALLING_R {
        WAIT_FOR_INTERRUPT_STALLING_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 0:23 - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 25:31 - 31:25\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&mut self) -> RESERVED28_W {
        RESERVED28_W { w: self }
    }
    #[doc = "Bit 24 - 24:24\\]
wait for interrupt stalling 0x0: Not supported 0x1: Wait for interrupt supported"]
    #[inline(always)]
    pub fn wait_for_interrupt_stalling(&mut self) -> WAIT_FOR_INTERRUPT_STALLING_W {
        WAIT_FOR_INTERRUPT_STALLING_W { w: self }
    }
    #[doc = "Bits 0:23 - 23:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
