#[doc = "Reader of register MISOCFG"]
pub type R = crate::R<u32, super::MISOCFG>;
#[doc = "Writer for register MISOCFG"]
pub type W = crate::W<u32, super::MISOCFG>;
#[doc = "Register MISOCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::MISOCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RESERVED5`"]
pub type RESERVED5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RESERVED5`"]
pub struct RESERVED5_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `AUXIO`"]
pub type AUXIO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AUXIO`"]
pub struct AUXIO_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bits 0:4 - 4:0\\]
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
    #[inline(always)]
    pub fn auxio(&self) -> AUXIO_R {
        AUXIO_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&mut self) -> RESERVED5_W {
        RESERVED5_W { w: self }
    }
    #[doc = "Bits 0:4 - 4:0\\]
AUXIO to MISO mux. Select the AUXIO pin that connects to MISO."]
    #[inline(always)]
    pub fn auxio(&mut self) -> AUXIO_W {
        AUXIO_W { w: self }
    }
}
