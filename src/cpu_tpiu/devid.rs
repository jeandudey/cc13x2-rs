#[doc = "Reader of register DEVID"]
pub type R = crate::R<u32, super::DEVID>;
#[doc = "Writer for register DEVID"]
pub type W = crate::W<u32, super::DEVID>;
#[doc = "Register DEVID `reset()`'s with value 0x0ca0"]
impl crate::ResetValue for super::DEVID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0ca0
    }
}
#[doc = "Reader of field `DEVID`"]
pub type DEVID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DEVID`"]
pub struct DEVID_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
    #[inline(always)]
    pub fn devid(&self) -> DEVID_R {
        DEVID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This field returns: 0xCA1 if there is an ETM present. 0xCA0 if there is no ETM present."]
    #[inline(always)]
    pub fn devid(&mut self) -> DEVID_W {
        DEVID_W { w: self }
    }
}
