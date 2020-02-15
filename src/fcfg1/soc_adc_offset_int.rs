#[doc = "Reader of register SOC_ADC_OFFSET_INT"]
pub type R = crate::R<u32, super::SOC_ADC_OFFSET_INT>;
#[doc = "Writer for register SOC_ADC_OFFSET_INT"]
pub type W = crate::W<u32, super::SOC_ADC_OFFSET_INT>;
#[doc = "Register SOC_ADC_OFFSET_INT `reset()`'s with value 0"]
impl crate::ResetValue for super::SOC_ADC_OFFSET_INT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED24`"]
pub type RESERVED24_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED24`"]
pub struct RESERVED24_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SOC_ADC_REL_OFFSET_TEMP1`"]
pub type SOC_ADC_REL_OFFSET_TEMP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOC_ADC_REL_OFFSET_TEMP1`"]
pub struct SOC_ADC_REL_OFFSET_TEMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_ADC_REL_OFFSET_TEMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RESERVED8`"]
pub type RESERVED8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED8`"]
pub struct RESERVED8_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SOC_ADC_ABS_OFFSET_TEMP1`"]
pub type SOC_ADC_ABS_OFFSET_TEMP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOC_ADC_ABS_OFFSET_TEMP1`"]
pub struct SOC_ADC_ABS_OFFSET_TEMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_ADC_ABS_OFFSET_TEMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_offset_temp1(&self) -> SOC_ADC_REL_OFFSET_TEMP1_R {
        SOC_ADC_REL_OFFSET_TEMP1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_abs_offset_temp1(&self) -> SOC_ADC_ABS_OFFSET_TEMP1_R {
        SOC_ADC_ABS_OFFSET_TEMP1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&mut self) -> RESERVED24_W {
        RESERVED24_W { w: self }
    }
    #[doc = "Bits 16:23 - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_offset_temp1(&mut self) -> SOC_ADC_REL_OFFSET_TEMP1_W {
        SOC_ADC_REL_OFFSET_TEMP1_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&mut self) -> RESERVED8_W {
        RESERVED8_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_abs_offset_temp1(&mut self) -> SOC_ADC_ABS_OFFSET_TEMP1_W {
        SOC_ADC_ABS_OFFSET_TEMP1_W { w: self }
    }
}
