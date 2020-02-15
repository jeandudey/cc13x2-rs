#[doc = "Reader of register ROM_TEST"]
pub type R = crate::R<u32, super::ROM_TEST>;
#[doc = "Writer for register ROM_TEST"]
pub type W = crate::W<u32, super::ROM_TEST>;
#[doc = "Register ROM_TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::ROM_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ROM_KEY`"]
pub type ROM_KEY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ROM_KEY`"]
pub struct ROM_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> ROM_KEY_W<'a> {
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
    pub fn rom_key(&self) -> ROM_KEY_R {
        ROM_KEY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom_key(&mut self) -> ROM_KEY_W {
        ROM_KEY_W { w: self }
    }
}
