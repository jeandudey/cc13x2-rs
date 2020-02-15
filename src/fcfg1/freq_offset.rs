#[doc = "Reader of register FREQ_OFFSET"]
pub type R = crate::R<u32, super::FREQ_OFFSET>;
#[doc = "Writer for register FREQ_OFFSET"]
pub type W = crate::W<u32, super::FREQ_OFFSET>;
#[doc = "Register FREQ_OFFSET `reset()`'s with value 0"]
impl crate::ResetValue for super::FREQ_OFFSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HPOSC_COMP_P0`"]
pub type HPOSC_COMP_P0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HPOSC_COMP_P0`"]
pub struct HPOSC_COMP_P0_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_COMP_P0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `HPOSC_COMP_P1`"]
pub type HPOSC_COMP_P1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPOSC_COMP_P1`"]
pub struct HPOSC_COMP_P1_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_COMP_P1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `HPOSC_COMP_P2`"]
pub type HPOSC_COMP_P2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HPOSC_COMP_P2`"]
pub struct HPOSC_COMP_P2_W<'a> {
    w: &'a mut W,
}
impl<'a> HPOSC_COMP_P2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p0(&self) -> HPOSC_COMP_P0_R {
        HPOSC_COMP_P0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p1(&self) -> HPOSC_COMP_P1_R {
        HPOSC_COMP_P1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p2(&self) -> HPOSC_COMP_P2_R {
        HPOSC_COMP_P2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p0(&mut self) -> HPOSC_COMP_P0_W {
        HPOSC_COMP_P0_W { w: self }
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p1(&mut self) -> HPOSC_COMP_P1_W {
        HPOSC_COMP_P1_W { w: self }
    }
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hposc_comp_p2(&mut self) -> HPOSC_COMP_P2_W {
        HPOSC_COMP_P2_W { w: self }
    }
}
