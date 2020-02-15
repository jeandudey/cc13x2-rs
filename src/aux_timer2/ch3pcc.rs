#[doc = "Reader of register CH3PCC"]
pub type R = crate::R<u32, super::CH3PCC>;
#[doc = "Writer for register CH3PCC"]
pub type W = crate::W<u32, super::CH3PCC>;
#[doc = "Register CH3PCC `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3PCC {
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
#[doc = "Reader of field `VALUE`"]
pub type VALUE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VALUE`"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
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
Pipeline Capture Compare value. 16-bit user defined pipeline compare value or channel-updated capture value. Compare mode: An update of VALUE will be transferred to CH3CC.VALUE when the next CNTR.VALUE is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When CH3EVCFG.CCACT equals PER_PULSE_WIDTH_MEAS then VALUE contains the width of the low or high phase of the selected signal. This is specified by CH3CCFG.EDGE and CH3CCFG.CAPT_SRC."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
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
Pipeline Capture Compare value. 16-bit user defined pipeline compare value or channel-updated capture value. Compare mode: An update of VALUE will be transferred to CH3CC.VALUE when the next CNTR.VALUE is zero and CTL.MODE is different from DIS. This is useful for PWM generation and prevents jitter on the edges of the generated signal. Capture mode: When CH3EVCFG.CCACT equals PER_PULSE_WIDTH_MEAS then VALUE contains the width of the low or high phase of the selected signal. This is specified by CH3CCFG.EDGE and CH3CCFG.CAPT_SRC."]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
}
