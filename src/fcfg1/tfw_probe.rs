#[doc = "Reader of register TFW_PROBE"]
pub type R = crate::R<u32, super::TFW_PROBE>;
#[doc = "Writer for register TFW_PROBE"]
pub type W = crate::W<u32, super::TFW_PROBE>;
#[doc = "Register TFW_PROBE `reset()`'s with value 0"]
impl crate::ResetValue for super::TFW_PROBE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REV`"]
pub type REV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REV`"]
pub struct REV_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_W<'a> {
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
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rev(&mut self) -> REV_W {
        REV_W { w: self }
    }
}
