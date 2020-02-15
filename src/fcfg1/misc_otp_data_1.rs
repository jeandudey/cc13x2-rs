#[doc = "Reader of register MISC_OTP_DATA_1"]
pub type R = crate::R<u32, super::MISC_OTP_DATA_1>;
#[doc = "Writer for register MISC_OTP_DATA_1"]
pub type W = crate::W<u32, super::MISC_OTP_DATA_1>;
#[doc = "Register MISC_OTP_DATA_1 `reset()`'s with value 0xe084_03f8"]
impl crate::ResetValue for super::MISC_OTP_DATA_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xe084_03f8
    }
}
#[doc = "Reader of field `PEAK_DET_ITRIM`"]
pub type PEAK_DET_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PEAK_DET_ITRIM`"]
pub struct PEAK_DET_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAK_DET_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `HP_BUF_ITRIM`"]
pub type HP_BUF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HP_BUF_ITRIM`"]
pub struct HP_BUF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HP_BUF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `LP_BUF_ITRIM`"]
pub type LP_BUF_ITRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LP_BUF_ITRIM`"]
pub struct LP_BUF_ITRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LP_BUF_ITRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `DBLR_LOOP_FILTER_RESET_VOLTAGE`"]
pub type DBLR_LOOP_FILTER_RESET_VOLTAGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBLR_LOOP_FILTER_RESET_VOLTAGE`"]
pub struct DBLR_LOOP_FILTER_RESET_VOLTAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBLR_LOOP_FILTER_RESET_VOLTAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `HPM_IBIAS_WAIT_CNT`"]
pub type HPM_IBIAS_WAIT_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HPM_IBIAS_WAIT_CNT`"]
pub struct HPM_IBIAS_WAIT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HPM_IBIAS_WAIT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | (((value as u32) & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Reader of field `LPM_IBIAS_WAIT_CNT`"]
pub type LPM_IBIAS_WAIT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_IBIAS_WAIT_CNT`"]
pub struct LPM_IBIAS_WAIT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_IBIAS_WAIT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 4)) | (((value as u32) & 0x3f) << 4);
        self.w
    }
}
#[doc = "Reader of field `IDAC_STEP`"]
pub type IDAC_STEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDAC_STEP`"]
pub struct IDAC_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAC_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&self) -> PEAK_DET_ITRIM_R {
        PEAK_DET_ITRIM_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&self) -> HP_BUF_ITRIM_R {
        HP_BUF_ITRIM_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&self) -> LP_BUF_ITRIM_R {
        LP_BUF_ITRIM_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dblr_loop_filter_reset_voltage(&self) -> DBLR_LOOP_FILTER_RESET_VOLTAGE_R {
        DBLR_LOOP_FILTER_RESET_VOLTAGE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&self) -> HPM_IBIAS_WAIT_CNT_R {
        HPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&self) -> LPM_IBIAS_WAIT_CNT_R {
        LPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idac_step(&self) -> IDAC_STEP_R {
        IDAC_STEP_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:28 - 28:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn peak_det_itrim(&mut self) -> PEAK_DET_ITRIM_W {
        PEAK_DET_ITRIM_W { w: self }
    }
    #[doc = "Bits 24:26 - 26:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hp_buf_itrim(&mut self) -> HP_BUF_ITRIM_W {
        HP_BUF_ITRIM_W { w: self }
    }
    #[doc = "Bits 22:23 - 23:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lp_buf_itrim(&mut self) -> LP_BUF_ITRIM_W {
        LP_BUF_ITRIM_W { w: self }
    }
    #[doc = "Bits 20:21 - 21:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dblr_loop_filter_reset_voltage(&mut self) -> DBLR_LOOP_FILTER_RESET_VOLTAGE_W {
        DBLR_LOOP_FILTER_RESET_VOLTAGE_W { w: self }
    }
    #[doc = "Bits 10:19 - 19:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&mut self) -> HPM_IBIAS_WAIT_CNT_W {
        HPM_IBIAS_WAIT_CNT_W { w: self }
    }
    #[doc = "Bits 4:9 - 9:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&mut self) -> LPM_IBIAS_WAIT_CNT_W {
        LPM_IBIAS_WAIT_CNT_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idac_step(&mut self) -> IDAC_STEP_W {
        IDAC_STEP_W { w: self }
    }
}
