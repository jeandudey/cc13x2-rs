#[doc = "Reader of register SOC_ADC_REF_TRIM_AND_OFFSET_EXT"]
pub type R = crate::R<u32, super::SOC_ADC_REF_TRIM_AND_OFFSET_EXT>;
#[doc = "Writer for register SOC_ADC_REF_TRIM_AND_OFFSET_EXT"]
pub type W = crate::W<u32, super::SOC_ADC_REF_TRIM_AND_OFFSET_EXT>;
#[doc = "Register SOC_ADC_REF_TRIM_AND_OFFSET_EXT `reset()`'s with value 0xc080"]
impl crate::ResetValue for super::SOC_ADC_REF_TRIM_AND_OFFSET_EXT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xc080
    }
}
#[doc = "Reader of field `RESERVED6`"]
pub type RESERVED6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED6`"]
pub struct RESERVED6_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff_ffff << 6)) | (((value as u32) & 0x03ff_ffff) << 6);
        self.w
    }
}
#[doc = "Reader of field `SOC_ADC_REF_VOLTAGE_TRIM_TEMP1`"]
pub type SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SOC_ADC_REF_VOLTAGE_TRIM_TEMP1`"]
pub struct SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 0x03ff_ffff) as u32)
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_adc_ref_voltage_trim_temp1(&self) -> SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_R {
        SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&mut self) -> RESERVED6_W {
        RESERVED6_W { w: self }
    }
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_adc_ref_voltage_trim_temp1(&mut self) -> SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_W {
        SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_W { w: self }
    }
}
