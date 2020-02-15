#[doc = "Reader of register RCOSCMFCTL"]
pub type R = crate::R<u32, super::RCOSCMFCTL>;
#[doc = "Writer for register RCOSCMFCTL"]
pub type W = crate::W<u32, super::RCOSCMFCTL>;
#[doc = "Register RCOSCMFCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::RCOSCMFCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPARE16`"]
pub type SPARE16_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SPARE16`"]
pub struct SPARE16_W<'a> {
    w: &'a mut W,
}
impl<'a> SPARE16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RCOSC_MF_CAP_ARRAY`"]
pub type RCOSC_MF_CAP_ARRAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSC_MF_CAP_ARRAY`"]
pub struct RCOSC_MF_CAP_ARRAY_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_MF_CAP_ARRAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `RCOSC_MF_REG_SEL`"]
pub type RCOSC_MF_REG_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCOSC_MF_REG_SEL`"]
pub struct RCOSC_MF_REG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_MF_REG_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RCOSC_MF_RES_COARSE`"]
pub type RCOSC_MF_RES_COARSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSC_MF_RES_COARSE`"]
pub struct RCOSC_MF_RES_COARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_MF_RES_COARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `RCOSC_MF_RES_FINE`"]
pub type RCOSC_MF_RES_FINE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSC_MF_RES_FINE`"]
pub struct RCOSC_MF_RES_FINE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_MF_RES_FINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RCOSC_MF_BIAS_ADJ`"]
pub type RCOSC_MF_BIAS_ADJ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSC_MF_BIAS_ADJ`"]
pub struct RCOSC_MF_BIAS_ADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_MF_BIAS_ADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&self) -> SPARE16_R {
        SPARE16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
    #[inline(always)]
    pub fn rcosc_mf_cap_array(&self) -> RCOSC_MF_CAP_ARRAY_R {
        RCOSC_MF_CAP_ARRAY_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
    #[inline(always)]
    pub fn rcosc_mf_reg_sel(&self) -> RCOSC_MF_REG_SEL_R {
        RCOSC_MF_REG_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
    #[inline(always)]
    pub fn rcosc_mf_res_coarse(&self) -> RCOSC_MF_RES_COARSE_R {
        RCOSC_MF_RES_COARSE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
    #[inline(always)]
    pub fn rcosc_mf_res_fine(&self) -> RCOSC_MF_RES_FINE_R {
        RCOSC_MF_RES_FINE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Adjusts bias current to RCOSC_MF. 0x8 minimum current 0x0 default current 0x7 maximum current"]
    #[inline(always)]
    pub fn rcosc_mf_bias_adj(&self) -> RCOSC_MF_BIAS_ADJ_R {
        RCOSC_MF_BIAS_ADJ_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&mut self) -> SPARE16_W {
        SPARE16_W { w: self }
    }
    #[doc = "Bits 9:15 - 15:9\\]
Adjust RCOSC_MF capacitor array. 0x0: nominal frequency, 0.625pF 0x40: highest frequency, 0.125pF 0x3F: lowest frequency, 1.125pF"]
    #[inline(always)]
    pub fn rcosc_mf_cap_array(&mut self) -> RCOSC_MF_CAP_ARRAY_W {
        RCOSC_MF_CAP_ARRAY_W { w: self }
    }
    #[doc = "Bit 8 - 8:8\\]
Choose regulator type. 0: default 1: alternate"]
    #[inline(always)]
    pub fn rcosc_mf_reg_sel(&mut self) -> RCOSC_MF_REG_SEL_W {
        RCOSC_MF_REG_SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - 7:6\\]
Select coarse resistor for frequency adjustment. 0x0: 400kohms, default 0x1: 300kohms, min 0x2: 600kohms, max 0x3: 500kohms"]
    #[inline(always)]
    pub fn rcosc_mf_res_coarse(&mut self) -> RCOSC_MF_RES_COARSE_W {
        RCOSC_MF_RES_COARSE_W { w: self }
    }
    #[doc = "Bits 4:5 - 5:4\\]
Select fine resistor for frequency adjustment. 0x0: 11kohms, minimum resistance, max freq 0x1: 13kohms 0x2: 16kohms 0x3: 20kohms, max resistance, min freq"]
    #[inline(always)]
    pub fn rcosc_mf_res_fine(&mut self) -> RCOSC_MF_RES_FINE_W {
        RCOSC_MF_RES_FINE_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Adjusts bias current to RCOSC_MF. 0x8 minimum current 0x0 default current 0x7 maximum current"]
    #[inline(always)]
    pub fn rcosc_mf_bias_adj(&mut self) -> RCOSC_MF_BIAS_ADJ_W {
        RCOSC_MF_BIAS_ADJ_W { w: self }
    }
}
