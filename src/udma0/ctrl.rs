#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BASEPTR`"]
pub type BASEPTR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BASEPTR`"]
pub struct BASEPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> BASEPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x003f_ffff << 10)) | (((value as u32) & 0x003f_ffff) << 10);
        self.w
    }
}
#[doc = "Reader of field `RESERVED0`"]
pub type RESERVED0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RESERVED0`"]
pub struct RESERVED0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 10:31 - 31:10\\]
This register point to the base address for the primary data structures of each DMA channel. This is not stored in module, but in system memory, thus space must be allocated for this usage when DMA is in usage"]
    #[inline(always)]
    pub fn baseptr(&self) -> BASEPTR_R {
        BASEPTR_R::new(((self.bits >> 10) & 0x003f_ffff) as u32)
    }
    #[doc = "Bits 0:9 - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 10:31 - 31:10\\]
This register point to the base address for the primary data structures of each DMA channel. This is not stored in module, but in system memory, thus space must be allocated for this usage when DMA is in usage"]
    #[inline(always)]
    pub fn baseptr(&mut self) -> BASEPTR_W {
        BASEPTR_W { w: self }
    }
    #[doc = "Bits 0:9 - 9:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&mut self) -> RESERVED0_W {
        RESERVED0_W { w: self }
    }
}
