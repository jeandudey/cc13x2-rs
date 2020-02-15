#[doc = "Reader of register AMPCOMP_CTRL1"]
pub type R = crate::R<u32, super::AMPCOMP_CTRL1>;
#[doc = "Writer for register AMPCOMP_CTRL1"]
pub type W = crate::W<u32, super::AMPCOMP_CTRL1>;
#[doc = "Register AMPCOMP_CTRL1 `reset()`'s with value 0xff48_3f47"]
impl crate::ResetValue for super::AMPCOMP_CTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xff48_3f47
    }
}
#[doc = "Reader of field `RESERVED1`"]
pub type RESERVED1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESERVED1`"]
pub struct RESERVED1_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `AMPCOMP_REQ_MODE`"]
pub type AMPCOMP_REQ_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AMPCOMP_REQ_MODE`"]
pub struct AMPCOMP_REQ_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPCOMP_REQ_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `IBIAS_OFFSET`"]
pub type IBIAS_OFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBIAS_OFFSET`"]
pub struct IBIAS_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `IBIAS_INIT`"]
pub type IBIAS_INIT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBIAS_INIT`"]
pub struct IBIAS_INIT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIAS_INIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `LPM_IBIAS_WAIT_CNT_FINAL`"]
pub type LPM_IBIAS_WAIT_CNT_FINAL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPM_IBIAS_WAIT_CNT_FINAL`"]
pub struct LPM_IBIAS_WAIT_CNT_FINAL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_IBIAS_WAIT_CNT_FINAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CAP_STEP`"]
pub type CAP_STEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAP_STEP`"]
pub struct CAP_STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `IBIASCAP_HPTOLP_OL_CNT`"]
pub type IBIASCAP_HPTOLP_OL_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IBIASCAP_HPTOLP_OL_CNT`"]
pub struct IBIASCAP_HPTOLP_OL_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> IBIASCAP_HPTOLP_OL_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&self) -> AMPCOMP_REQ_MODE_R {
        AMPCOMP_REQ_MODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_offset(&self) -> IBIAS_OFFSET_R {
        IBIAS_OFFSET_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_init(&self) -> IBIAS_INIT_R {
        IBIAS_INIT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LPM_IBIAS_WAIT_CNT_FINAL_R {
        LPM_IBIAS_WAIT_CNT_FINAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cap_step(&self) -> CAP_STEP_R {
        CAP_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IBIASCAP_HPTOLP_OL_CNT_R {
        IBIASCAP_HPTOLP_OL_CNT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&mut self) -> RESERVED1_W {
        RESERVED1_W { w: self }
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&mut self) -> AMPCOMP_REQ_MODE_W {
        AMPCOMP_REQ_MODE_W { w: self }
    }
    #[doc = "Bits 24:29 - 29:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_offset(&mut self) -> IBIAS_OFFSET_W {
        IBIAS_OFFSET_W { w: self }
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_init(&mut self) -> IBIAS_INIT_W {
        IBIAS_INIT_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&mut self) -> LPM_IBIAS_WAIT_CNT_FINAL_W {
        LPM_IBIAS_WAIT_CNT_FINAL_W { w: self }
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cap_step(&mut self) -> CAP_STEP_W {
        CAP_STEP_W { w: self }
    }
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&mut self) -> IBIASCAP_HPTOLP_OL_CNT_W {
        IBIASCAP_HPTOLP_OL_CNT_W { w: self }
    }
}
