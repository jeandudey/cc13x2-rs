#[doc = "Reader of register DAC_CAL1"]
pub type R = crate::R<u32, super::DAC_CAL1>;
#[doc = "Writer for register DAC_CAL1"]
pub type W = crate::W<u32, super::DAC_CAL1>;
#[doc = "Register DAC_CAL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DAC_CAL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SOC_DAC_VOUT_CAL_PRECH_C2`"]
pub type SOC_DAC_VOUT_CAL_PRECH_C2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SOC_DAC_VOUT_CAL_PRECH_C2`"]
pub struct SOC_DAC_VOUT_CAL_PRECH_C2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_DAC_VOUT_CAL_PRECH_C2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SOC_DAC_VOUT_CAL_PRECH_C1`"]
pub type SOC_DAC_VOUT_CAL_PRECH_C1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SOC_DAC_VOUT_CAL_PRECH_C1`"]
pub struct SOC_DAC_VOUT_CAL_PRECH_C1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOC_DAC_VOUT_CAL_PRECH_C1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_prech_c2(&self) -> SOC_DAC_VOUT_CAL_PRECH_C2_R {
        SOC_DAC_VOUT_CAL_PRECH_C2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_prech_c1(&self) -> SOC_DAC_VOUT_CAL_PRECH_C1_R {
        SOC_DAC_VOUT_CAL_PRECH_C1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_prech_c2(&mut self) -> SOC_DAC_VOUT_CAL_PRECH_C2_W {
        SOC_DAC_VOUT_CAL_PRECH_C2_W { w: self }
    }
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_prech_c1(&mut self) -> SOC_DAC_VOUT_CAL_PRECH_C1_W {
        SOC_DAC_VOUT_CAL_PRECH_C1_W { w: self }
    }
}
