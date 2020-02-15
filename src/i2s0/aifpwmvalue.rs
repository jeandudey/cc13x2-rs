#[doc = "Reader of register AIFPWMVALUE"]
pub type R = crate::R<u32, super::AIFPWMVALUE>;
#[doc = "Writer for register AIFPWMVALUE"]
pub type W = crate::W<u32, super::AIFPWMVALUE>;
#[doc = "Register AIFPWMVALUE `reset()`'s with value 0"]
impl crate::ResetValue for super::AIFPWMVALUE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED16`"]
pub type RESERVED16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED16`"]
pub struct RESERVED16_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PULSE_WIDTH`"]
pub type PULSE_WIDTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PULSE_WIDTH`"]
pub struct PULSE_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSE_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
    #[inline(always)]
    pub fn pulse_width(&self) -> PULSE_WIDTH_R {
        PULSE_WIDTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&mut self) -> RESERVED16_W {
        RESERVED16_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
The value written to this register determines the width of the active high PWM pulse (pwm_debug), which starts together with MSB of the first output word in a DMA buffer: 0x0000: Constant low 0x0001: Width of the pulse (number of BCLK cycles, here 1). ... 0xFFFE: Width of the pulse (number of BCLK cycles, here 65534). 0xFFFF: Constant high"]
    #[inline(always)]
    pub fn pulse_width(&mut self) -> PULSE_WIDTH_W {
        PULSE_WIDTH_W { w: self }
    }
}
