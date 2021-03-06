#[doc = "Reader of register DMACH1EXTADDR"]
pub type R = crate::R<u32, super::DMACH1EXTADDR>;
#[doc = "Writer for register DMACH1EXTADDR"]
pub type W = crate::W<u32, super::DMACH1EXTADDR>;
#[doc = "Register DMACH1EXTADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACH1EXTADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface. Note: The crypto DMA copies out upto 3 bytes until it hits a word boundary, thus the address need not be word aligned."]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel external address value. When read during operation, it holds the last updated external address after being sent to the master interface. Note: The crypto DMA copies out upto 3 bytes until it hits a word boundary, thus the address need not be word aligned."]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
