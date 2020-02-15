#[doc = "Reader of register MISC_OTP_DATA"]
pub type R = crate::R<u32, super::MISC_OTP_DATA>;
#[doc = "Writer for register MISC_OTP_DATA"]
pub type W = crate::W<u32, super::MISC_OTP_DATA>;
#[doc = "Register MISC_OTP_DATA `reset()`'s with value 0xcf00"]
impl crate::ResetValue for super::MISC_OTP_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xcf00
    }
}
#[doc = "Reader of field `RCOSC_HF_ITUNE`"]
pub type RCOSC_HF_ITUNE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSC_HF_ITUNE`"]
pub struct RCOSC_HF_ITUNE_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_HF_ITUNE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Reader of field `RCOSC_HF_CRIM`"]
pub type RCOSC_HF_CRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RCOSC_HF_CRIM`"]
pub struct RCOSC_HF_CRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RCOSC_HF_CRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 20)) | (((value as u32) & 0xff) << 20);
        self.w
    }
}
#[doc = "Reader of field `PER_M`"]
pub type PER_M_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER_M`"]
pub struct PER_M_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | (((value as u32) & 0x1f) << 15);
        self.w
    }
}
#[doc = "Reader of field `PER_E`"]
pub type PER_E_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PER_E`"]
pub struct PER_E_W<'a> {
    w: &'a mut W,
}
impl<'a> PER_E_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `TEST_PROGRAM_REV`"]
pub type TEST_PROGRAM_REV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TEST_PROGRAM_REV`"]
pub struct TEST_PROGRAM_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_PROGRAM_REV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_itune(&self) -> RCOSC_HF_ITUNE_R {
        RCOSC_HF_ITUNE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 20:27 - 27:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_crim(&self) -> RCOSC_HF_CRIM_R {
        RCOSC_HF_CRIM_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline(always)]
    pub fn test_program_rev(&self) -> TEST_PROGRAM_REV_R {
        TEST_PROGRAM_REV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_itune(&mut self) -> RCOSC_HF_ITUNE_W {
        RCOSC_HF_ITUNE_W { w: self }
    }
    #[doc = "Bits 20:27 - 27:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_crim(&mut self) -> RCOSC_HF_CRIM_W {
        RCOSC_HF_CRIM_W { w: self }
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&mut self) -> PER_M_W {
        PER_M_W { w: self }
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&mut self) -> PER_E_W {
        PER_E_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline(always)]
    pub fn test_program_rev(&mut self) -> TEST_PROGRAM_REV_W {
        TEST_PROGRAM_REV_W { w: self }
    }
}
