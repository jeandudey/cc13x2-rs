#[doc = "Reader of register SOC_ADC_REL_GAIN"]
pub type R = crate::R<u32, super::SOC_ADC_REL_GAIN>;
#[doc = "Writer for register SOC_ADC_REL_GAIN"]
pub type W = crate::W<u32, super::SOC_ADC_REL_GAIN>;
#[doc = "Register SOC_ADC_REL_GAIN `reset()`'s with value 0"]
impl crate::ResetValue for super::SOC_ADC_REL_GAIN {
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
#[doc = "Reader of field `SOC_ADC_REL_GAIN_TEMP1`"]
pub type SOC_ADC_REL_GAIN_TEMP1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SOC_ADC_REL_GAIN_TEMP1`"]
pub struct SOC_ADC_REL_GAIN_TEMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_ADC_REL_GAIN_TEMP1_W<'a> {
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
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_gain_temp1(&self) -> SOC_ADC_REL_GAIN_TEMP1_R {
        SOC_ADC_REL_GAIN_TEMP1_R::new((self.bits & 0xffff) as u16)
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
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_gain_temp1(&mut self) -> SOC_ADC_REL_GAIN_TEMP1_W {
        SOC_ADC_REL_GAIN_TEMP1_W { w: self }
    }
}
