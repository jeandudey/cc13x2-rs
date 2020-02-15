#[doc = "Reader of register FREQ_OFFSET"]
pub type R = crate::R<u32, super::FREQ_OFFSET>;
#[doc = "Writer for register FREQ_OFFSET"]
pub type W = crate::W<u32, super::FREQ_OFFSET>;
#[doc = "Register FREQ_OFFSET `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::FREQ_OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `HF_COMP_P0`"]
pub type HF_COMP_P0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HF_COMP_P0`"]
pub struct HF_COMP_P0_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_COMP_P0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HF_COMP_P1`"]
pub type HF_COMP_P1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HF_COMP_P1`"]
pub struct HF_COMP_P1_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_COMP_P1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HF_COMP_P2`"]
pub type HF_COMP_P2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HF_COMP_P2`"]
pub struct HF_COMP_P2_W<'a> {
    w: &'a mut W,
}
impl<'a> HF_COMP_P2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p0(&self) -> HF_COMP_P0_R {
        HF_COMP_P0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p1(&self) -> HF_COMP_P1_R {
        HF_COMP_P1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p2(&self) -> HF_COMP_P2_R {
        HF_COMP_P2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p0(&mut self) -> HF_COMP_P0_W {
        HF_COMP_P0_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p1(&mut self) -> HF_COMP_P1_W {
        HF_COMP_P1_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p2(&mut self) -> HF_COMP_P2_W {
        HF_COMP_P2_W { w: self }
    }
}
