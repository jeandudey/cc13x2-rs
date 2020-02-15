#[doc = "Reader of register FEDACCTL2"]
pub type R = crate::R<u32, super::FEDACCTL2>;
#[doc = "Writer for register FEDACCTL2"]
pub type W = crate::W<u32, super::FEDACCTL2>;
#[doc = "Register FEDACCTL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FEDACCTL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC_THRESHOLD`"]
pub type SEC_THRESHOLD_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEC_THRESHOLD`"]
pub struct SEC_THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_THRESHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sec_threshold(&self) -> SEC_THRESHOLD_R {
        SEC_THRESHOLD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sec_threshold(&mut self) -> SEC_THRESHOLD_W {
        SEC_THRESHOLD_W { w: self }
    }
}
